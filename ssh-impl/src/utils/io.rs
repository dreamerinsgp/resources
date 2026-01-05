use std::io::{Read, Write};
use anyhow::{Result, Context};

/// Read exactly `n` bytes from a reader
pub fn read_exact(reader: &mut dyn Read, n: usize) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; n];
    reader.read_exact(&mut buf)
        .context(format!("Failed to read {} bytes", n))?;
    Ok(buf)
}

/// Read a line terminated by CRLF (\r\n)
pub fn read_line_crlf(reader: &mut dyn Read) -> Result<String> {
    let mut buf = Vec::new();
    let mut prev_byte = None;
    
    loop {
        let mut byte = [0u8; 1];
        reader.read_exact(&mut byte)
            .context("Failed to read line")?;
        
        if byte[0] == b'\n' && prev_byte == Some(b'\r') {
            // Found CRLF, remove the \r from buffer
            buf.pop();
            break;
        }
        
        buf.push(byte[0]);
        prev_byte = Some(byte[0]);
    }
    
    String::from_utf8(buf)
        .context("Invalid UTF-8 in line")
}

/// Write data to a writer
pub fn write_all(writer: &mut dyn Write, data: &[u8]) -> Result<()> {
    writer.write_all(data)
        .context("Failed to write data")?;
    writer.flush()
        .context("Failed to flush writer")?;
    Ok(())
}

