use crate::{Algorithm, OperationMode, OutputFormat};
use cipher_core::{BlockCipher, CipherResult};

#[derive(Clone)]
pub struct CipherContext {
    pub algorithm: Algorithm,
    pub operation: OperationMode,
    pub key: String,
    pub input_text: String,
    pub output_format: OutputFormat,
}

impl CipherContext {
    #[inline]
    #[must_use]
    pub const fn new(
        algorithm: Algorithm,
        operation: OperationMode,
        key: String,
        input_text: String,
        output_format: OutputFormat,
    ) -> Self {
        Self {
            algorithm,
            operation,
            key,
            input_text,
            output_format,
        }
    }

    /// # Errors
    ///
    /// Returns `Err` if parsing the input text or creating the cipher fails,
    /// or if the encryption/decryption process encounters an error.
    pub fn process(&self) -> CipherResult<String> {
        let text_bytes = self.algorithm.parse_text(&self.input_text)?;
        let cipher = self.algorithm.new_cipher(&self.key)?;
        self.execute(cipher.as_ref(), &text_bytes)
    }

    fn execute(&self, cipher: &dyn BlockCipher, text_bytes: &[u8]) -> CipherResult<String> {
        match self.operation {
            OperationMode::Encrypt => {
                let ciphertext = cipher.encrypt(text_bytes)?;
                Ok(format!("{ciphertext:X}"))
            }
            OperationMode::Decrypt => {
                let plaintext = cipher.decrypt(text_bytes)?;
                let output = self.output_format.format(&plaintext);
                Ok(output)
            }
        }
    }
}
