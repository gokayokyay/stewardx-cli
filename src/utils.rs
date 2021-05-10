use std::{process, str::FromStr};

use serde_json::Value;

pub fn parse_cron_frequency(frequency: &str) -> String {
    let cron_str = frequency.starts_with("Every(").then(|| remove_cron_freq_prefix(frequency)).or_else(|| Some(frequency));
    let cron_str = match cron_str {
        Some(c) => c,
        None => {
            eprintln!("Please enter a valid cron string.");
            process::exit(1);
        }
    };
    cron_str.split(" ").collect::<Vec<&str>>().len().ne(&6).then(||{
        eprintln!("Please enter a valid cron string. Hint: StewardX's cron frequency needs to take 6 crontime inputs, like * * * * * *");
        process::exit(1)
    });
    match cron::Schedule::from_str(cron_str) {
        Ok(_a) => {},
        Err(_e) => {
            eprintln!("Please enter a valid cron string.");
            process::exit(1);
        }
    };
    let parsed_frequency = format!("Every({})", cron_str);
    return parsed_frequency;   
}

pub fn remove_cron_freq_prefix(frequency: &str) -> &str {
    let mut chars = frequency.chars();
    while let Some(a) = chars.next() {
        if a.to_string() == "(" {
            break;
        }
    }
    chars.next_back();
    chars.as_str()
}

pub fn print_connection_failure(e: isahc::Error) {
    log::debug!("{}", e);
    eprintln!(r#"
        Couldn't connect to StewardX. Here's what you can do:
        - Try the same command with setting LOG_LEVEL environment variable to debug, like LOG_LEVEL=debug stewardx ...
        - Check if StewardX instance is running
        - Check environment variables, STEWARDX_URL or STEWARDX_HOST and STEWARDX_PORT
        - Use cURL to connect StewardX instance, if it doesn't fail, please open an issue at https://github.com/gokayokyay/stewardx-cli
    "#);
}

pub fn print_json_failure(e: serde_json::Error) {
    log::error!("Error while trying to parse response from StewardX, this shouldn't happen, please open an issue at https://github.com/gokayokyay/stewardx-cli");
    log::error!("{}", e);
}

pub fn print_invalid_task_value(id: &str, key: &str, value: &Value) {
    log::error!("Task with id: {} has an invalid property named: \"{}\" with value: \"{}\"", id, key, value);
    log::error!("Please open an issue at https://github.com/gokayokyay/stewardx-cli and describe how to reproduce it, thanks!");
}

pub fn pretty_print_tasks(tasks: Vec<Value>) {
    println!("{0: <12} | {1: <12} | {2: <12} | {3: <12}", "Task ID", "Name", "Type", "Frequency");
    for task in tasks {
        pretty_print_task(task);
    }
}

pub fn pretty_print_task(task: Value) {
    // If id doesn't exist, then yeah, you can panic
    let id = &task["id"].as_str().unwrap().to_string();
    let task_name = match &task["task_name"] {
        Value::String(v) => v.to_string(),
        _ => {
            print_invalid_task_value(id, "task_name", &task["task_name"]);
            process::exit(1);
        }
    };
    let task_type = match &task["task_type"] {
        Value::String(v) => v.to_string(),
        _ => {
            print_invalid_task_value(id, "task_type", &task["task_type"]);
            process::exit(1);
        }
    };
    let frequency = match &task["frequency"] {
        Value::String(v) => v.to_string(),
        _ => {
            print_invalid_task_value(id, "frequency", &task["frequency"]);
            process::exit(1);
        }
    };
    println!("{0: <12} | {1: <12} | {2: <12} | {3: <12}", id, task_name, task_type, frequency);
}
