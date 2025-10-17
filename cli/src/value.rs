use crate::error::ValueError;
use std::{
    fmt::{Display, LowerHex, UpperHex},
    fs::read_to_string,
    path::PathBuf,
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
pub struct Value(u64);

impl Value {
    #[inline]
    #[must_use]
    pub const fn as_64(self) -> u64 {
        self.0
    }

    #[inline]
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 8] {
        self.0.to_be_bytes()
    }
}

impl From<Value> for u64 {
    fn from(value: Value) -> Self {
        value.as_64()
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl FromStr for Value {
    type Err = ValueError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse::<u64>() {
            return Ok(Self(num));
        }

        let path = PathBuf::from(s);
        if path.exists() && path.is_file() {
            if let Ok(contents) = read_to_string(&path) {
                let value = parse_string_to_u64(&contents)?;
                return Ok(Self(value));
            }
            return Err(ValueError::FileReadingError(path));
        }

        let value = parse_string_to_u64(s)?;
        Ok(Self(value))
    }
}

fn parse_string_to_u64(s: &str) -> Result<u64, ValueError> {
    let trimmed = s.trim();

    if trimmed.is_empty() {
        return Err(ValueError::EmptyString);
    }

    // Hexadecimal with 0x/0X prefix
    if let Some(hex_str) = trimmed
        .strip_prefix("0X")
        .or_else(|| trimmed.strip_prefix("0x"))
    {
        return parse_radix(hex_str, 16, "Hex");
    }

    // Binary with 0b/0B prefix
    if let Some(bin_str) = trimmed
        .strip_prefix("0b")
        .or_else(|| trimmed.strip_prefix("0B"))
    {
        return parse_radix(bin_str, 2, "Binary");
    }

    // 8-character ASCII string conversion to u64
    if trimmed.len() > 8 {
        return Err(ValueError::InvalidByteStringLength(trimmed.len()));
    }

    ascii_string_to_u64(trimmed)
}

fn parse_radix(s: &str, radix: u32, name: &str) -> Result<u64, ValueError> {
    let trimmed = s.trim_start_matches('0');
    if trimmed.is_empty() {
        return Ok(0);
    }

    u64::from_str_radix(trimmed, radix)
        .map_err(|e| ValueError::InvalidFormat(format!("{name} parsing failed: {e}")))
}

fn ascii_string_to_u64(s: &str) -> Result<u64, ValueError> {
    if !s.is_ascii() {
        return Err(ValueError::ConversionError(
            "String contains non-ASCII characters".into(),
        ));
    }

    let mut bytes = [0; 8];
    for (idx, byte) in s.bytes().enumerate() {
        bytes[idx] = byte;
    }

    Ok(u64::from_be_bytes(bytes))
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0b}", self.0)
    }
}

impl UpperHex for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016X}", self.0)
    }
}

impl LowerHex for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}
