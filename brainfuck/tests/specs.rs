use assert_cmd::Command;
use std::fs;

fn run_spec(name: &'static str) {
    let root = format!("specs/{name}");
    let mut command = Command::cargo_bin("bf").unwrap();
    command.arg("-f").arg(format!("{root}.code.bf"));
    if let Ok(input) = fs::read_to_string(format!("{root}.in.txt")) {
        command.write_stdin(input);
    }
    let assert = command.assert().success();
    if let Ok(output) = fs::read_to_string(format!("{root}.out.txt")) {
        assert.stdout(output);
    }
}

#[test]
fn add_works() {
    run_spec("add")
}

#[test]
fn cat_works() {
    run_spec("cat")
}

#[test]
fn even_works() {
    run_spec("even")
}

#[test]
fn fibonacci_works() {
    run_spec("fibonacci")
}

#[test]
fn hello_golf_works() {
    run_spec("hello-golf")
}

#[test]
fn hello_normal_works() {
    run_spec("hello-normal")
}

#[ignore = "this is currently too slow"]
#[test]
fn mandelbrot_works() {
    run_spec("mandelbrot")
}
