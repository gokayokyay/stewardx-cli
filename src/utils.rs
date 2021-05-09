use std::{process, str::FromStr};

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