//! A very simple Task Manager

use std::process;

mod command_line_interface;
mod task;
mod tasklist;

fn main() {
    if let Err(e) = command_line_interface::run(None) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
