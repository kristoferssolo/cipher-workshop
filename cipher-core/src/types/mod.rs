mod output;

pub use output::Output;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherAction {
    Encrypt,
    Decrypt,
}
