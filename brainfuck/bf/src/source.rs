pub(crate) fn highlight_source(header: &str, source: &str, pos: usize, message: &str) -> String {
    let (line, col) = line_col(source, pos);
    format!(
        "{header}\n{}\n  |        at {line}:{col}",
        highlight_code(source, pos, message)
    )
}

pub(crate) fn highlight_code(source: &str, pos: usize, message: &str) -> String {
    let padded = &format!("     {source}      ")[pos..pos + 11].replace('\n', "␤");
    format!("  | {padded}\n  |      ^ {message}")
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

#[cfg(test)]
mod tests {
    mod highlight_source {
        use indoc::indoc;

        use super::super::highlight_source;

        #[test]
        fn correctly_highlights_source() {
            assert_eq!(
                highlight_source(
                    "Error: My error message.",
                    "# my comment\n++[-].",
                    14,
                    "message"
                ),
                indoc! {"
                    Error: My error message.
                      | ent␤++[-]. 
                      |      ^ message
                      |        at 2:2"}
            )
        }
    }

    mod line_col {
        use super::super::line_col;

        #[test]
        fn returns_1_1_for_first_character_in_source() {
            assert_eq!(line_col("a", 0), (1, 1));
        }

        #[test]
        fn correctly_increments_column_numbers() {
            assert_eq!(line_col("a bcd 123", 6), (1, 7));
        }

        #[test]
        fn correctly_increments_line_numbers() {
            assert_eq!(line_col("a\nbcd\n123", 6), (3, 1));
        }

        #[test]
        fn treats_newline_as_the_last_character_on_the_line() {
            assert_eq!(line_col("a\nb", 1), (1, 2));
        }

        #[test]
        fn does_not_count_control_characters() {
            assert_eq!(line_col("a\n\rb", 3), (2, 1));
        }
    }
}
