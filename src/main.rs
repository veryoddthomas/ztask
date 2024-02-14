use std::process;

mod cli;
mod task;
mod tasklist;

fn main() {
    if let Err(e) = cli::run(None) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
