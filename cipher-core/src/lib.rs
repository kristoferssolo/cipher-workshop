mod error;
mod macros;
mod padding;
mod parsing;
mod traits;
mod types;

pub use {
    error::{BlockError, CipherError, CipherResult},
    padding::{pkcs7_pad, pkcs7_unpad},
    parsing::{BlockInt, parse_block_int},
    traits::{BlockCipher, BlockParser, InputBlock},
    types::{CipherAction, Output},
};

pub mod prelude {
    pub use super::{CipherAction, CipherResult, InputBlock, Output};
}
