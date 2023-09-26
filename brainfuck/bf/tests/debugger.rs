use predicates::str;
use util::bf;

mod util;

#[test]
fn stops_at_breakpoint() {
    bf().arg("-f")
        .arg("specs/add.code.bf")
        .arg("-d")
        .assert()
        .stderr(str::starts_with(
            "Debugger: Entering debugger due to a breakpoint hit.",
        ));
}

#[test]
fn can_inspect_memory() {
    bf().arg("-c")
        .arg("+>++>+++>++++>+++++#")
        .arg("-d")
        .write_stdin(":m 2")
        .assert()
        .stderr(str::contains("[001] [002] [003] [004] [005]"));
}

#[test]
fn shows_surrounding_code_on_breakpoint_hit() {
    bf().arg("-c")
        .arg("abc#def")
        .arg("-d")
        .assert()
        .stderr(str::contains("abc#def"));
}
