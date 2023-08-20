use std::io::{Read, Write};

#[must_use]
pub fn read_u8<R: Read>(read: &mut R) -> Option<u8> {
    let mut buffer = [0];
    read.read_exact(&mut buffer).ok()?;
    Some(buffer[0])
}

#[must_use]
pub fn write_u8<W: Write>(write: &mut W, value: u8) -> Option<()> {
    write.write_all(&[value]).ok()
}
