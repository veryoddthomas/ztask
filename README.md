# ZTask

- [CLI Project from Rust Book](https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html)
- [Command line apps in Rust](https://rust-cli.github.io/book/index.html)
- [Coverage](https://doc.rust-lang.org/rustc/instrument-coverage.html)



## Basic Execution

```bash
cargo run -q -- -v -l
```

## Test Coverage

### Install tarpaulin for coverage

Until I deal with this, make sure there is an empty
database when setting up a new project dir:

data/db.json:
```json
[]
```


```bash
apt install libssl-dev
cargo install cargo-tarpaulin
```

### Run tests and measure coverage

Pick one

```bash
cargo test
cargo tarpaulin --implicit-test-threads
cargo tarpaulin --implicit-test-threads --out Html && wslview tarpaulin-report.html
```

## Checkers

```bash
cargo clippy --all --all-targets -- -D warnings
# note, this runs cargo check
```

## Documentation

### Public

```bash
cargo doc --no-deps --open
```

### Private

```bash
cargo doc --document-private-items --no-deps --open
```

## To Do

- Investigate https://docs.rs/mockall/latest/mockall/