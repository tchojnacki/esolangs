use assert_cmd::Command;

pub fn bf() -> Command {
    Command::cargo_bin("bf").expect("Unable to create the bf command!")
}
