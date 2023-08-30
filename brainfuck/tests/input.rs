use assert_cmd::Command;

#[test]
fn file_can_be_used_as_input() {
    Command::cargo_bin("brainfuck")
        .unwrap()
        .arg("-f")
        .arg("tests/specs/cat.code.bf")
        .write_stdin("Hello, world!")
        .assert()
        .success()
        .stdout("Hello, world!");
}

#[test]
fn code_can_be_used_as_input() {
    Command::cargo_bin("brainfuck")
        .unwrap()
        .arg("-c")
        .arg(",[.,]")
        .write_stdin("Hello, world!")
        .assert()
        .success()
        .stdout("Hello, world!");
}

#[test]
fn stdin_can_be_used_as_input() {
    Command::cargo_bin("brainfuck")
        .unwrap()
        .arg("-s")
        .write_stdin(",[.,]!Hello, world!")
        .assert()
        .success()
        .stdout("Hello, world!");
}

#[test]
fn file_arg_requires_value() {
    Command::cargo_bin("brainfuck")
        .unwrap()
        .arg("-f")
        .assert()
        .failure();
}

#[test]
fn code_arg_requires_value() {
    Command::cargo_bin("brainfuck")
        .unwrap()
        .arg("-c")
        .assert()
        .failure();
}

#[test]
fn input_methods_are_mutually_exclusive() {
    Command::cargo_bin("brainfuck")
        .unwrap()
        .arg("-s")
        .arg("-c")
        .arg(",[.,]")
        .assert()
        .failure();
}
