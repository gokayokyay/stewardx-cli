use std::{os::unix::prelude::PermissionsExt, process};

use isahc::{ReadResponseExt, Request, RequestExt, config::{Configurable, RedirectPolicy}};
use serde_json::{Result as SerdeResult, Value};

use crate::{output::{print_connection_failure, print_json_failure}, utils::{create_stewardx_dirs, get_binary_dir, get_nodejs_compatible_arch}};

pub fn check_os_and_arch(name: &str) -> bool {
    let os = std::env::consts::OS;
    let arch = get_nodejs_compatible_arch();
    let os_arch = format!("{}_{}", os, arch);
    name.contains(&os_arch)
}

pub fn fetch_latest_binary() {
    match isahc::get("https://api.github.com/repos/gokayokyay/stewardx/releases/latest") {
        Ok(mut r) => {
            let resp: SerdeResult<Value> = r.json();
            let resp = match resp {
                Ok(r) => r,
                Err(e) => {
                    print_json_failure(e);
                    process::exit(1);
                }
            };
            let assets = match &resp["assets"].as_array() {
                Some(a) => *a,
                None => {
                    println!("Latest release doesn't have any assets. Please manually check: \"https://github.com/gokayokyay/stewardx/releases/latest\"");
                    process::exit(1);
                }
            };
            for asset in assets {
                let name = asset["name"].as_str().unwrap();
                if !check_os_and_arch(name) {
                    continue;
                }
                println!("Found the matching binary! Downloading it...");
                let download_url = asset["browser_download_url"].as_str().unwrap();
                create_stewardx_dirs();
                let request = Request::get(download_url).redirect_policy(RedirectPolicy::Follow).body(()).unwrap();
                match request.send() {
                    Ok(mut o) => {
                        let mut binary_dir = get_binary_dir();
                        binary_dir.push(name);
                        match o.copy_to_file(binary_dir.clone()) {
                            Ok(_) => {
                                let mut perms = std::fs::metadata(binary_dir.clone()).unwrap().permissions();
                                perms.set_mode(0o700);
                                std::fs::set_permissions(binary_dir.clone(), perms).unwrap();
                                println!("Fetched latest binary for your platform! It's located at: {}", binary_dir.to_str().unwrap());
                            }
                            Err(e) => {
                                eprintln!("Error while downloading the latest binary.");
                                panic!("{}", e);
                            }
                        };
                    }
                    Err(e) => {
                        print_connection_failure(e);
                        process::exit(1)
                    }
                }
                break;
            }
        }
        Err(e) => {
            print_connection_failure(e);
            process::exit(1);
        }
    };
}

