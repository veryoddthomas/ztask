use std::process;

fn main() {
    if let Err(e) = ztask::run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use std::fs;

    #[test]
    fn invoke_main() {
        super::main();
    }

    #[test]
    fn invoke_main_1() {
        let test_db = "data/test.json";
        let _ = fs::remove_file(test_db);

        let mut cmd = Command::cargo_bin("ztask").unwrap();
        let assert = cmd
            .arg("--db").arg(test_db)
            .arg("-l")
            .arg("-v")
            // .arg("-a")
            .assert();
        assert
            .success()
            .code(0);
            //.stdout(predicate::str::contains("task count: "));
    }

    // #[test]
    // fn test_main() {
    //     let mut cmd = Command::cargo_bin("ztask").unwrap();
    //     cmd.assert().success();
    // }

    // #[test]
    // fn test_version() {
    //     let mut cmd = Command::cargo_bin("ztask").unwrap();
    //     cmd.arg("--version").assert().success();
    // }

    // #[test]
    // fn test_help() {
    //     let mut cmd = Command::cargo_bin("ztask").unwrap();
    //     cmd.arg("--help").assert().success();
    // }

    // #[test]
    // fn test_main_1() {
    //     let mut cmd = assert_cmd::cargo::cargo_bin("ztask");
    //     println!("{:?}", cmd.as_os_str());
    //     let assert = cmd
    //         .arg("-A")
    //         .env("stdout", "hello")
    //         .env("exit", "42")
    //         .write_stdin("42")
    //         .assert();
    //     assert
    //         .failure()
    //         .code(42)
    //         .stdout("hello\n");

    // }
}
