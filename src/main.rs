use std::process;

mod cli;
mod task;

fn main() {
    if let Err(e) = cli::run(None) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn invoke_main() {
        super::main();
    }
}
