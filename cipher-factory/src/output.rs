use cipher_core::Output;

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
    pub fn to_string(&self, value: &Output) -> String {
        match self {
            Self::Binary => format!("{value:b}"),
            Self::Octal => format!("{value:o}"),
            Self::Hex => format!("{value:X}"),
            Self::Text => format!("{value}"),
        }
    }
}
