use std::io::{ErrorKind, Read, Write};

/// Read a single byte from a given [`Read`], e.g. the standard input.
///
/// This returns [`Some`] if a byte was read and [`None`] on any error.
/// EOF is treated as a correct byte with a value of `0`.
///
/// # Examples
/// ```
/// # use brainlib::util::read_byte;
/// let byte = read_byte("".as_bytes()).ok_or("error while reading")?;
/// println!("Read: {}", byte);
/// // => Read: 0
/// # assert_eq!(byte, 0);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[must_use]
pub fn read_byte(mut input: impl Read) -> Option<u8> {
    let mut buffer = [0];
    match input.read_exact(&mut buffer).map_err(|e| e.kind()) {
        Ok(_) => Some(buffer[0]),
        Err(ErrorKind::UnexpectedEof) => Some(0),
        _ => None,
    }
}

/// Write a single byte to a given [`Write`], e.g. the standard output.
///
/// This returns [`Some`] if the byte was written and [`None`] on any error.
///
/// # Examples
/// ```
/// # use brainlib::util::write_byte;
/// let mut output = Vec::new();
/// write_byte(&mut output, b'A').ok_or("error while writing")?;
/// # assert_eq!(output, vec![b'A']);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[must_use]
pub fn write_byte(mut output: impl Write, value: u8) -> Option<()> {
    output.write_all(&[value]).ok()?;
    output.flush().ok()
}
