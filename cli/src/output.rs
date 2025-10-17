use clap::ValueEnum;

#[derive(Debug, Clone, Default, ValueEnum)]
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
