use std::{process, str::FromStr};

use serde_json::Value;

use crate::utils::{format_date, truncate_string_elliptic};

pub fn print_connection_failure(e: isahc::Error) {
    log::debug!("{}", e);
    eprintln!(
        r#"
        Connection failed, please check your network status and run the same command with LOG_LEVEL=debug
        "#
    );
}

pub fn print_stewardx_connection_failure(e: isahc::Error) {
    log::debug!("{}", e);
    eprintln!(
        r#"
        Couldn't connect to StewardX. Here's what you can do:
        - Try the same command with setting LOG_LEVEL environment variable to debug, like LOG_LEVEL=debug stewardx ...
        - Check if StewardX instance is running
        - Check environment variables, STEWARDX_URL or STEWARDX_HOST and STEWARDX_PORT
        - Use cURL to connect StewardX instance, if it doesn't fail, please open an issue at https://github.com/gokayokyay/stewardx-cli
    "#
    );
}

pub fn print_json_failure(e: serde_json::Error) {
    log::error!("Error while trying to parse response from StewardX, this shouldn't happen, please open an issue at https://github.com/gokayokyay/stewardx-cli");
    log::error!("{}", e);
}

fn format_and_print_report(id: &str, task_id: &str, executed_at: &str, successful: impl ToString) {
    println!(
        "{0: <36} | {1: <36} | {2: <24} | {3: <10}",
        id,
        task_id,
        executed_at,
        successful.to_string()
    );
}

pub fn pretty_print_reports(reports: Vec<Value>) {
    format_and_print_report("Report ID", "Task ID", "Executed At", "Did success");
    println!("---------------------------------------------------------------------------------------------------------------------");
    for report in reports {
        parse_and_print_report(report);
    }
}

pub fn print_invalid_report_value(id: &str, key: &str, value: &Value) {
    log::error!(
        "Report with id: {} has an invalid property named: \"{}\" with value: \"{}\"",
        id,
        key,
        value
    );
    log::error!("Please open an issue at https://github.com/gokayokyay/stewardx-cli and describe how to reproduce it, thanks!");
}

pub fn parse_and_print_report(report: Value) {
    // If id doesn't exist, then yeah, you can panic
    let id = &report["id"].as_str().unwrap().to_string();
    let task_id = match &report["task_id"] {
        Value::String(v) => v.to_string(),
        _ => {
            print_invalid_report_value(id, "task_id", &report["task_id"]);
            process::exit(1);
        }
    };
    let created_at = match &report["created_at"] {
        Value::String(v) => match chrono::NaiveDateTime::from_str(v) {
            Ok(o) => format_date(o),
            Err(e) => {
                log::error!("{}", e);
                print_invalid_report_value(id, "created_at", &report["created_at"]);
                process::exit(1);
            }
        },
        _ => {
            print_invalid_report_value(id, "created_at", &report["created_at"]);
            process::exit(1);
        }
    };
    let successful = match &report["successful"] {
        Value::Bool(v) => v,
        _ => {
            print_invalid_report_value(id, "successful", &report["successful"]);
            process::exit(1);
        }
    };
    format_and_print_report(id, &task_id, &created_at, *successful);
}

fn format_and_print_task(id: &str, name: &str, task_type: &str, frequency: &str) {
    println!(
        "{0: <36} | {1: <16} | {2: <8} | {3: <16}",
        id, name, task_type, frequency
    );
}

pub fn pretty_print_tasks(tasks: Vec<Value>) {
    format_and_print_task("Task ID", "Name", "Type", "Frequency");
    println!("---------------------------------------------------------------------------------");
    for task in tasks {
        parse_and_print_task(task);
    }
}

pub fn parse_and_print_task(task: Value) {
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
    let task_name = truncate_string_elliptic(task_name, 16);
    format_and_print_task(id, &task_name, &task_type, &frequency);
}

pub fn print_invalid_task_value(id: &str, key: &str, value: &Value) {
    log::error!(
        "Task with id: {} has an invalid property named: \"{}\" with value: \"{}\"",
        id,
        key,
        value
    );
    log::error!("Please open an issue at https://github.com/gokayokyay/stewardx-cli and describe how to reproduce it, thanks!");
}
