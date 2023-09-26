use util::bf;

mod util;

#[test]
fn file_can_be_used_as_input() {
    bf().arg("-f")
        .arg("specs/cat.code.bf")
        .write_stdin("Hello, world!")
        .assert()
        .success()
        .stdout("Hello, world!");
}

#[test]
fn code_can_be_used_as_input() {
    bf().arg("-c")
        .arg(",[.,]")
        .write_stdin("Hello, world!")
        .assert()
        .success()
        .stdout("Hello, world!");
}

#[test]
fn stdin_can_be_used_as_input() {
    bf().arg("-s")
        .write_stdin(",[.,]!Hello, world!")
        .assert()
        .success()
        .stdout("Hello, world!");
}

#[test]
fn file_arg_requires_value() {
    bf().arg("-f").assert().failure();
}

#[test]
fn code_arg_requires_value() {
    bf().arg("-c").assert().failure();
}

#[test]
fn input_methods_are_mutually_exclusive() {
    bf().arg("-s").arg("-c").arg(",[.,]").assert().failure();
}
