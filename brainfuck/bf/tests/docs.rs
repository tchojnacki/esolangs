use util::bf;

mod util;

#[test]
fn binary_examples() {
    bf().arg("--help").assert().success();

    // TODO: this is too slow
    // Command::cargo_bin("bf")
    //     .unwrap()
    //     .arg("-f")
    //     .arg("specs/mandelbrot.code.bf")
    //     .assert()
    //     .success();

    bf().arg("-c")
        .arg("++>+++++[<+>-]++++++++[<++++++>-]<.")
        .assert()
        .success();
}
