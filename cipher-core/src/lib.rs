mod error;
mod parsing;
mod traits;
mod types;

pub use {
    error::{BlockError, CipherError, CipherResult},
    parsing::{parse_block_int, BlockInt},
    traits::{BlockCipher, BlockParser, InputBlock},
    types::{CipherAction, Output},
};

pub mod prelude {
    pub use super::{CipherAction, CipherResult, InputBlock, Output};
}
