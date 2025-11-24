use std::fmt::Display;

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    Des,
    Aes,
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Des => "Des",
            Self::Aes => "Aes",
        };
        f.write_str(s)
    }
}
