use anyhow::Result;
use assert_cmd::Command;

const PRG_NAME: &str = "swaybinds";

fn set_home(home_dir: &str) {
    std::env::set_var("HOME", format!("tests/inputs/{home_dir}"));
}

#[test]
fn error_when_no_key_bindings() -> Result<()> {
    set_home("no_keybinds");
    Command::cargo_bin(PRG_NAME)?
        .assert()
        .failure()
        .stderr("Can't find key bindings in this file.\n");
    Ok(())
}

#[test]
fn error_when_no_key_bindings_custom_config() -> Result<()> {
    Command::cargo_bin(PRG_NAME)?
        .args(vec!["-c", "tests/inputs/no_keybinds/custom/config"])
        .assert()
        .failure()
        .stderr("Can't find key bindings in this file.\n");
    Ok(())
}

#[test]
fn ok_when_key_bindings() -> Result<()> {
    set_home("keybinds");
    let expected = std::fs::read_to_string("tests/expected/table")?;
    Command::cargo_bin(PRG_NAME)?
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn ok_when_key_bindings_custom_config() -> Result<()> {
    let expected = std::fs::read_to_string("tests/expected/table")?;
    Command::cargo_bin(PRG_NAME)?
        .args(vec!["--config", "tests/inputs/keybinds/custom/config"])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
