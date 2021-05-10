use std::process;

use isahc::{get, prelude::*};
use serde_json::{Result as SerdeResult, Value};

use crate::utils::{pretty_print_tasks, print_connection_failure, print_json_failure};

fn get_stewardx_url() -> String {
    match std::env::var("STEWARDX_URL") {
        Ok(url) => url,
        Err(_) => {
            let host = std::env::var("STEWARDX_HOST").unwrap_or("localhost".into());
            let port = std::env::var("STEWARDX_PORT").unwrap_or("3000".into());
            format!("http://{}:{}", host, port)
        }
    }
}

pub fn get_tasks() {
    let url = format!("{}/tasks", get_stewardx_url());
    let tasks: Result<SerdeResult<Value>, isahc::Error> = get(url).map(|mut t| t.json());
    let tasks = match tasks {
        Ok(a) => {
            match a {
                Ok(val) => val,
                Err(e) => {
                    print_json_failure(e);
                    process::exit(1);
                }
            }
        },
        Err(e) => {
            print_connection_failure(e);
            process::exit(1);
        }
    };
    let tasks = tasks.as_array().map(|v| v.to_owned()).unwrap_or(Vec::new());
    pretty_print_tasks(tasks.to_vec());
}