mod api;
mod output;
mod utils;

use std::fs;
use std::process;

use api::{
    abort_task, delete_task, execute_task, get_active_tasks, get_latest_reports,
    get_reports_for_task,
};
use clap::{load_yaml, App, ArgMatches};
use env_logger::Env;

use crate::{
    api::{create_task, get_report, get_task, get_tasks},
    utils::{capitalize, parse_cron_frequency},
};

fn handle_tasks(tasks: &ArgMatches) {
    if let Some(list) = tasks.subcommand_matches("list") {
        if let Some(task_id) = list.value_of("ID") {
            get_task(task_id);
        } else {
            get_tasks();
        }
    }
    if let Some(create) = tasks.subcommand_matches("create") {
        if let Some(cmd) = create.subcommand_matches("cmd") {
            let command = cmd.value_of("command").unwrap();
            let name = cmd.value_of("name").unwrap();
            let frequency = cmd.value_of("frequency").unwrap();
            let frequency = if frequency == "Hook" {
                frequency.to_string()
            } else {
                parse_cron_frequency(frequency)
            };
            let props = serde_json::json!({ "command": command });
            create_task("CmdTask", name, &frequency, &props);
        } else if let Some(docker) = create.subcommand_matches("docker") {
            let name = docker.value_of("name").unwrap();
            let frequency = docker.value_of("frequency").unwrap();
            let frequency = if frequency == "Hook" {
                frequency.to_string()
            } else {
                parse_cron_frequency(frequency)
            };
            let docker_type = docker.value_of("type").unwrap();
            let contents = docker.value_of("contents").unwrap();
            let environment_vars = docker
                .values_of("env")
                .unwrap_or(clap::Values::default())
                .collect::<Vec<&str>>();
            let contents = match docker_type {
                "file" => match fs::read_to_string(contents) {
                    Ok(c) => c,
                    Err(_e) => {
                        eprintln!("Couldn't read the file specified, please make sure the Dockerfile's path is correct.");
                        process::exit(1);
                    }
                },
                "image" => contents.to_string(),
                _ => {
                    eprintln!("Invalid type specified, please supply either \"file\" or \"image\"");
                    process::exit(1);
                }
            };
            let docker_type = capitalize(docker_type);
            let task_props = serde_json::json!({
                "image": {
                    "t": docker_type,
                    "c": contents
                },
                "env": environment_vars
            });
            create_task("DockerTask", name, &frequency, &task_props);
        } else {
            eprintln!("Error: please supply either cmd or docker to create command");
            process::exit(1);
        }
    }
    if let Some(_) = tasks.subcommand_matches("active") {
        get_active_tasks();
    }
    if let Some(delete) = tasks.subcommand_matches("delete") {
        let task_id = delete.value_of("ID").unwrap();
        delete_task(task_id);
    }
    if let Some(execute) = tasks.subcommand_matches("execute") {
        let task_id = execute.value_of("ID").unwrap();
        execute_task(task_id);
    }
    if let Some(abort) = tasks.subcommand_matches("abort") {
        let task_id = abort.value_of("ID").unwrap();
        abort_task(task_id);
    }
}

fn handle_reports(reports: &ArgMatches) {
    if let Some(list) = reports.subcommand_matches("list") {
        if let Some(report_id) = list.value_of("ID") {
            get_report(report_id);
        } else {
            get_latest_reports();
        }
    }
    if let Some(task) = reports.value_of("task") {
        get_reports_for_task(task);
    }
    if let Some(_latest) = reports.subcommand_matches("latest") {
        get_latest_reports()
    }
}

fn main() {
    let env = Env::default().filter_or("LOG_LEVEL", "info");
    env_logger::init_from_env(env);
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    // println!("{:?}", matches);
    if let Some(tasks) = matches.subcommand_matches("tasks") {
        handle_tasks(tasks);
    }
    if let Some(reports) = matches.subcommand_matches("reports") {
        handle_reports(reports);
    }
}
