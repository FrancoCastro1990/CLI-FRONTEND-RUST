use assert_cmd::Command;
use predicates::prelude::*;

fn get_cli_command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_cli-frontend"))
}

#[test]
fn test_cli_version() {
    let mut cmd = get_cli_command();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_cli_help() {
    let mut cmd = get_cli_command();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("CLI Frontend Generator"))
        .stdout(predicate::str::contains("--type"))
        .stdout(predicate::str::contains("--list"));
}

#[test]
fn test_cli_list() {
    let mut cmd = get_cli_command();
    cmd.arg("--list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Available Templates:"));
}

#[test]
fn test_cli_describe_component() {
    let mut cmd = get_cli_command();
    cmd.arg("--describe").arg("component");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Template:"));
}

#[test]
fn test_cli_invalid_template() {
    let mut cmd = get_cli_command();
    cmd.arg("TestComponent")
        .arg("--type")
        .arg("nonexistent_template");

    cmd.assert().failure();
}

#[test]
fn test_cli_missing_name() {
    let mut cmd = get_cli_command();
    cmd.arg("--type").arg("component");

    // Should fail because no name was provided
    cmd.assert().failure();
}

#[test]
fn test_cli_with_var_flag() {
    let mut cmd = get_cli_command();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--var"));
}
