use assert_cmd::Command;

#[test]
fn binary_examples() {
    Command::cargo_bin("bf")
        .unwrap()
        .arg("--help")
        .assert()
        .success();

    // TODO: this is too slow
    // Command::cargo_bin("bf")
    //     .unwrap()
    //     .arg("-f")
    //     .arg("specs/mandelbrot.code.bf")
    //     .assert()
    //     .success();

    Command::cargo_bin("bf")
        .unwrap()
        .arg("-c")
        .arg("++>+++++[<+>-]++++++++[<++++++>-]<.")
        .assert()
        .success();
}
