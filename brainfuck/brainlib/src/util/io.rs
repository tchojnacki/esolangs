use std::io::{ErrorKind, Read, Write};

#[must_use]
pub fn read_byte(mut input: impl Read) -> Option<u8> {
    let mut buffer = [0];
    match input.read_exact(&mut buffer).map_err(|e| e.kind()) {
        Ok(_) => Some(buffer[0]),
        Err(ErrorKind::UnexpectedEof) => Some(0),
        _ => None,
    }
}

#[must_use]
pub fn write_byte(mut output: impl Write, value: u8) -> Option<()> {
    output.write_all(&[value]).ok()?;
    output.flush().ok()
}
