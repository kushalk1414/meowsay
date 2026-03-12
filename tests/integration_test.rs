use std::process::Command;
use assert_cmd::prelude::*;

#[test]fn fail_on_non_existing_file()
-> Result<(), Box<dyn std::error::Error>> {
Command::cargo_bin("meowsay")
.expect("binary exists")
.args(&["-f", "no/such/file.txt"])
.assert()
.failure();
Ok(())
}