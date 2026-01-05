use std::io::{Read, Write};

/// Trait for types that can be used as both Read and Write
pub trait ReadWrite: Read + Write {}
impl<T: Read + Write> ReadWrite for T {}

