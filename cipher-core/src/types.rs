use std::{
    fmt::{Binary, Display, LowerHex, Octal, UpperHex},
    ops::Deref,
    str::from_utf8,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherAction {
    Encrypt,
    Decrypt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CipherOutput(Vec<u8>);

impl CipherOutput {
    #[inline]
    #[must_use]
    pub fn new(value: &[u8]) -> Self {
        Self(value.to_vec())
    }
}

impl UpperHex for CipherOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.0 {
            write!(f, "{byte:02X}")?;
        }
        Ok(())
    }
}

impl LowerHex for CipherOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl Octal for CipherOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.0 {
            write!(f, "{byte:03o}")?;
        }
        Ok(())
    }
}

impl Binary for CipherOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.0 {
            write!(f, "{byte:08b}")?;
        }
        Ok(())
    }
}

impl Display for CipherOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match from_utf8(&self.0) {
            Ok(s) => f.write_str(s),
            Err(_) => write!(f, "{self:X}"),
        }
    }
}

impl Deref for CipherOutput {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for CipherOutput
where
    T: Into<Vec<u8>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
