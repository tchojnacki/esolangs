use std::io::{ErrorKind, Read, Write};

#[must_use]
pub fn read_u8<R: Read>(read: &mut R) -> Option<u8> {
    let mut buffer = [0];
    match read.read_exact(&mut buffer).map_err(|e| e.kind()) {
        Ok(_) => Some(buffer[0]),
        Err(ErrorKind::UnexpectedEof) => Some(0),
        _ => None,
    }
}

#[must_use]
pub fn write_u8<W: Write>(write: &mut W, value: u8) -> Option<()> {
    write.write_all(&[value]).ok()
}
