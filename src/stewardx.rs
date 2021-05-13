use std::{os::unix::prelude::PermissionsExt, process};

use isahc::{ReadResponseExt, Request, RequestExt, config::{Configurable, Dialer, RedirectPolicy}};
use serde_json::{Result as SerdeResult, Value};

use crate::{output::{print_connection_failure, print_json_failure}, utils::{create_stewardx_dirs, get_binary_dir, get_nodejs_compatible_arch, get_socket_path}};

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
                        binary_dir.push("stewardx");
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

pub fn start_stewardx() {
    // Check STEWARDX_DATABASE_URL env var
    match std::env::var("STEWARDX_DATABASE_URL") {
        Ok(_) => {
            println!("✓ - STEWARDX_DATABASE_URL environment variable has been found.")
        },
        Err(_) => {
            eprintln!("X - STEWARDX_DATABASE_URL environment variable doesn't exist. Quitting.");
            process::exit(1);
        }
    };
    use fork::{daemon, Fork};
    use std::process::Command;
    let mut binary_path = get_binary_dir();
    binary_path.push("stewardx");
    if let Ok(Fork::Child) = daemon(false, false) {
        Command::new(binary_path)
            .output()
            .expect("failed to execute process");
        println!("Started StewardX!");
    }
}

pub fn stop_stewardx() {
    let request = Request::get("http://stop")
        .dial(Dialer::unix_socket(get_socket_path().to_str().unwrap()))
        .body(()).unwrap();
    match request.send() {
        Ok(mut r) => {
            let response = r.text().unwrap();
            if response.eq("Goodbye!") {
                println!("Successfully stopped StewardX.");
            } else {
                println!("StewardX returned other than a goodbye message, here it is: {}", response);
            }
        }
        Err(e) => {
            println!("{}", e.to_string());
            print_connection_failure(e);
        }
    };
}