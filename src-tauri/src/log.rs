use chrono::{DateTime, Local};
use std::fs::File;
use std::io::Write;

use crate::file_storage;

pub fn error(message: &str) {
    log(message, "ERROR");
}

pub fn info(message: &str) {
    log(message, "INFO");
}

fn log(message: &str, level: &str) {
    let current_local: DateTime<Local> = Local::now();
    let formatted_message = &format!("{} [{}] {}", current_local, level, message);
    if cfg!(dev) {
        log_to_console(formatted_message);
    }
    log_to_file(formatted_message);
}

fn log_to_file(message: &str) {
    let file_path = file_storage::get_logs_path();

    if !file_path.exists() {
        let parent_dir = file_path.parent().unwrap();
        std::fs::create_dir_all(parent_dir).unwrap();
        std::fs::write(&file_path, "").unwrap();
    }

    let mut f = File::options().append(true).open(file_path).unwrap();
    writeln!(&mut f, "{}", message).unwrap_or_else(|_| log_to_console(message));
}

fn log_to_console(message: &str) {
    println!("{}", message);
}
