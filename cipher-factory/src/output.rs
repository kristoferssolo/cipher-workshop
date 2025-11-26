use cipher_core::Output;
use std::{convert::Infallible, fmt::Display, str::FromStr};
use strum::EnumIter;

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumIter)]
pub enum OutputFormat {
    /// Binary output
    Binary,
    /// Octal output
    Octal,
    /// Decimal output
    #[default]
    Hex,
    /// Text output (ASCII)
    Text,
}

impl OutputFormat {
    #[must_use]
    pub fn format(&self, value: &Output) -> String {
        match self {
            Self::Binary => format!("{value:b}"),
            Self::Octal => format!("{value:o}"),
            Self::Hex => format!("{value:X}"),
            Self::Text => format!("{value}"),
        }
    }
}

impl FromStr for OutputFormat {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim().to_lowercase().as_ref() {
            "binary" | "bin" => Self::Binary,
            "octal" | "oct" => Self::Octal,
            "text" | "txt" => Self::Text,
            _ => Self::Hex,
        })
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Binary => "Binary",
            Self::Octal => "Octal",
            Self::Hex => "Hexadecimal",
            Self::Text => "Text",
        };
        f.write_str(s)
    }
}
