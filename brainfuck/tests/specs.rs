use std::fs;
use util::bf;

mod util;

fn run_spec(name: &'static str) {
    let root = format!("specs/{name}");
    let mut command = bf();
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
fn add_spec() {
    run_spec("add")
}

#[test]
fn cat_spec() {
    run_spec("cat")
}

#[test]
fn even_spec() {
    run_spec("even")
}

#[test]
fn fibonacci_spec() {
    run_spec("fibonacci")
}

#[test]
fn hello_golf_spec() {
    run_spec("hello-golf")
}

#[test]
fn hello_normal_spec() {
    run_spec("hello-normal")
}

#[ignore = "this is currently too slow"]
#[test]
fn mandelbrot_spec() {
    run_spec("mandelbrot")
}
