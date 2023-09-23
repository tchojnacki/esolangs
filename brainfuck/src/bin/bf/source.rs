pub fn highlight_source(header: &str, source: &str, pos: usize, message: &str) -> String {
    let (line, col) = line_col(source, pos);
    format!(
        "{header}\n{}  |        at {line}:{col}",
        highlight_code(source, pos, message)
    )
}

pub fn highlight_code(source: &str, pos: usize, message: &str) -> String {
    let padded = &format!("     {source}      ")[pos..pos + 11].replace('\n', "␤");
    format!("  | {padded}\n  |      ^ {message}\n")
}

fn line_col(source: &str, pos: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for c in source[..pos].chars() {
        if c == '\n' {
            line += 1;
            col = 1;
        } else if !c.is_control() {
            col += 1;
        }
    }
    (line, col)
}
