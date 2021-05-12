use std::process;

use isahc::{get, post, prelude::*, Request};
use serde_json::{Result as SerdeResult, Value};

use crate::output::{
    pretty_print_reports, pretty_print_tasks, print_stewardx_connection_failure, print_json_failure,
};

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

pub fn get_active_tasks() {
    let url = format!("{}/activetasks", get_stewardx_url());
    let tasks: Result<SerdeResult<Value>, isahc::Error> = get(url).map(|mut t| t.json());
    let tasks = match tasks {
        Ok(a) => match a {
            Ok(val) => val,
            Err(e) => {
                print_json_failure(e);
                process::exit(1);
            }
        },
        Err(e) => {
            print_stewardx_connection_failure(e);
            process::exit(1);
        }
    };
    let tasks = tasks.as_array().map(|v| v.to_owned()).unwrap_or(Vec::new());
    pretty_print_tasks(tasks.to_vec());
}

pub fn get_tasks() {
    let url = format!("{}/tasks", get_stewardx_url());
    let tasks: Result<SerdeResult<Value>, isahc::Error> = get(url).map(|mut t| t.json());
    let tasks = match tasks {
        Ok(a) => match a {
            Ok(val) => val,
            Err(e) => {
                print_json_failure(e);
                process::exit(1);
            }
        },
        Err(e) => {
            print_stewardx_connection_failure(e);
            process::exit(1);
        }
    };
    let tasks = tasks.as_array().map(|v| v.to_owned()).unwrap_or(Vec::new());
    pretty_print_tasks(tasks.to_vec());
}

pub fn get_task(id: &str) {
    let url = format!("{}/tasks/{}", get_stewardx_url(), id);
    let tasks: Result<SerdeResult<Value>, isahc::Error> = get(url).map(|mut t| t.json());
    let task = match tasks {
        Ok(a) => match a {
            Ok(val) => val,
            Err(e) => {
                print_json_failure(e);
                process::exit(1);
            }
        },
        Err(e) => {
            print_stewardx_connection_failure(e);
            process::exit(1);
        }
    };
    match serde_json::to_string_pretty(&task) {
        Ok(o) => {
            println!("{}", o);
        }
        Err(e) => {
            print_json_failure(e);
            process::exit(1);
        }
    };
}

pub fn delete_task(id: &str) {
    let url = format!("{}/tasks", get_stewardx_url());
    let request = Request::builder()
        .uri(url)
        .method(isahc::http::Method::DELETE)
        .body(serde_json::json!({ "task_id": id }).to_string())
        .unwrap();
    let response = request.send();
    match response {
        Ok(mut r) => {
            let result: SerdeResult<Value> = r.json();
            match result {
                Ok(r) => {
                    let status = &r["status"];
                    if let Some(status) = status.as_str() {
                        println!("Task deletion status: {}", status);
                    } else {
                        println!("Task deletion is failed, please check StewardX logs.");
                    }
                }
                Err(e) => {
                    print_json_failure(e);
                    process::exit(1);
                }
            };
        }
        Err(e) => {
            print_stewardx_connection_failure(e);
            process::exit(1);
        }
    };
}

pub fn execute_task(id: &str) {
    let url = format!("{}/execute/{}", get_stewardx_url(), id);
    let task: Result<SerdeResult<Value>, isahc::Error> = post(url, ()).map(|mut t| t.json());
    match task {
        Ok(result) => {
            match result {
                Ok(r) => {
                    let status = &r["status"];
                    if let Some(status) = status.as_str() {
                        println!("Task execution status: {}", status);
                    } else {
                        println!("Task execution is failed, please check StewardX logs.");
                    }
                }
                Err(e) => {
                    print_json_failure(e);
                    process::exit(1);
                }
            };
        }
        Err(e) => {
            print_stewardx_connection_failure(e);
            process::exit(1);
        }
    };
}

pub fn abort_task(id: &str) {
    let url = format!("{}/abort", get_stewardx_url());
    let request = Request::builder()
        .uri(url)
        .method(isahc::http::Method::POST)
        .body(serde_json::json!({ "task_id": id }).to_string())
        .unwrap();
    let response = request.send();
    match response {
        Ok(mut r) => {
            let result: SerdeResult<Value> = r.json();
            match result {
                Ok(r) => {
                    let status = &r["status"];
                    if let Some(status) = status.as_str() {
                        println!("Task abortion status: {}", status);
                    } else {
                        println!("Task abortion is failed, please check StewardX logs.");
                    }
                }
                Err(e) => {
                    print_json_failure(e);
                    process::exit(1);
                }
            };
        }
        Err(e) => {
            print_stewardx_connection_failure(e);
            process::exit(1);
        }
    };
}

pub fn create_task(task_type: &str, name: &str, frequency: &str, props: &Value) {
    let url = format!("{}/tasks", get_stewardx_url());
    let request = Request::builder()
        .uri(url)
        .method(isahc::http::Method::POST)
        .body(
            serde_json::json!({
                "task_type": task_type,
                "task_name": name,
                "frequency": frequency,
                "task_props": props
            })
            .to_string(),
        )
        .unwrap();
    let response = request.send();
    match response {
        Ok(mut r) => {
            let result: SerdeResult<Value> = r.json();
            match result {
                Ok(r) => {
                    pretty_print_tasks(vec![r]);
                }
                Err(e) => {
                    print_json_failure(e);
                    process::exit(1);
                }
            };
        }
        Err(e) => {
            print_stewardx_connection_failure(e);
            process::exit(1);
        }
    };
}

pub fn get_reports_for_task(id: &str) {
    let url = format!("{}/task/{}/reports", get_stewardx_url(), id);
    let reports: Result<SerdeResult<Value>, isahc::Error> = get(url).map(|mut t| t.json());
    match reports {
        Ok(result) => {
            match result {
                Ok(r) => {
                    let reports = r.as_array().map(|v| v.to_owned()).unwrap_or(Vec::new());
                    pretty_print_reports(reports);
                    // println!("{}", r);
                }
                Err(e) => {
                    print_json_failure(e);
                    process::exit(1);
                }
            };
        }
        Err(e) => {
            print_stewardx_connection_failure(e);
            process::exit(1);
        }
    };
}

pub fn get_latest_reports() {
    let url = format!("{}/reports", get_stewardx_url());
    let reports: Result<SerdeResult<Value>, isahc::Error> = get(url).map(|mut t| t.json());
    match reports {
        Ok(result) => {
            match result {
                Ok(r) => {
                    let reports = r.as_array().map(|v| v.to_owned()).unwrap_or(Vec::new());
                    pretty_print_reports(reports);
                    // println!("{}", r);
                }
                Err(e) => {
                    print_json_failure(e);
                    process::exit(1);
                }
            };
        }
        Err(e) => {
            print_stewardx_connection_failure(e);
            process::exit(1);
        }
    };
}

pub fn get_report(id: &str) {
    let url = format!("{}/reports/{}", get_stewardx_url(), id);
    let reports: Result<SerdeResult<Value>, isahc::Error> = get(url).map(|mut t| t.json());
    match reports {
        Ok(result) => {
            match result {
                Ok(r) => match serde_json::to_string_pretty(&r) {
                    Ok(v) => {
                        println!("{}", v);
                    }
                    Err(e) => {
                        print_json_failure(e);
                        process::exit(1);
                    }
                },
                Err(e) => {
                    print_json_failure(e);
                    process::exit(1);
                }
            };
        }
        Err(e) => {
            print_stewardx_connection_failure(e);
            process::exit(1);
        }
    };
}
