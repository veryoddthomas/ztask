// use std::env;
use std::process;
// use ztask::*;



fn main() {
    if let Err(e) = ztask::run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}

#[cfg(test)]
mod tests {
    #[test]
    // #[ignore]  // Not ready yet
    fn invoke_main() {
        super::main();
    }
}
