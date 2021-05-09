mod utils;
mod api;
use std::process;
use std::fs;

use clap::{App, ArgMatches, load_yaml};

use crate::utils::parse_cron_frequency;

fn handle_tasks(tasks: &ArgMatches) {
    if let Some(list) = tasks.subcommand_matches("list") {
        println!("Listing tasks");
    }
    if let Some(create) = tasks.subcommand_matches("create") {
        println!("Creating task: {:?}", create);
        if let Some(cmd) = create.subcommand_matches("cmd") {
            let command = cmd.value_of("command").unwrap();
            let name = cmd.value_of("name").unwrap();
            let frequency = cmd.value_of("frequency").unwrap();
            let frequency = if frequency == "Hook" {
                frequency.to_string()
            } else {
                parse_cron_frequency(frequency)
            };
            println!("{}, {}, {}", name,command,frequency);
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
            let contents = match docker_type {
                "file" => {
                    match fs::read_to_string(contents) {
                        Ok(c) => c,
                        Err(_e) => {
                            eprintln!("Couldn't read the file specified, please make sure the Dockerfile's path is correct.");
                            process::exit(1);
                        }
                    }
                },
                "image" => {
                    contents.to_string()
                }
                _ => {
                    eprintln!("Invalid type specified, please supply either \"file\" or \"image\"");
                    process::exit(1);
                }
            };
            println!("{}, {}, {}, {}", name, frequency, docker_type, contents);
        } else {
            eprintln!("Error: please supply either cmd or docker to create command");
            process::exit(1);
        }
    }
}

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    // println!("{:?}", matches);
    if let Some(tasks) = matches.subcommand_matches("tasks") {
        handle_tasks(tasks);
    }
}