mod error;
mod traits;
mod types;

pub use {
    error::{BlockError, CipherError, CipherResult},
    traits::{BlockCipher, BlockParser, InputBlock},
    types::{CipherAction, Output},
};

pub mod prelude {
    pub use super::{CipherAction, CipherResult, InputBlock, Output};
}
