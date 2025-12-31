use crate::{Algorithm, OperationMode, OutputFormat};
use cipher_core::{BlockCipher, CipherError, CipherResult, Output};

#[derive(Clone)]
pub struct CipherContext {
    pub algorithm: Algorithm,
    pub operation: OperationMode,
    pub key: String,
    pub iv: Option<String>,
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
        iv: Option<String>,
        input_text: String,
        output_format: OutputFormat,
    ) -> Self {
        Self {
            algorithm,
            operation,
            key,
            iv,
            input_text,
            output_format,
        }
    }

    /// Processes the input text using the configured cipher algorithm and operation.
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// - Parsing the input text or creating the cipher fails
    /// - The encryption/decryption process encounters an error
    /// - CBC mode is used without providing an IV
    pub fn process(&self) -> CipherResult<String> {
        if self.algorithm.requires_iv() {
            self.process_cbc()
        } else {
            self.process_ecb()
        }
    }

    fn process_ecb(&self) -> CipherResult<String> {
        let text_bytes = self.algorithm.parse_text(&self.input_text)?;
        let cipher = self.algorithm.new_cipher(&self.key)?;
        self.execute_ecb(cipher.as_ref(), &text_bytes)
    }

    fn process_cbc(&self) -> CipherResult<String> {
        let iv = self
            .iv
            .as_ref()
            .ok_or_else(|| CipherError::InvalidPadding("CBC mode requires an IV".into()))?;

        let cipher = self.algorithm.new_cbc_cipher(&self.key, iv)?;

        match self.operation {
            OperationMode::Encrypt => {
                let ciphertext = cipher.encrypt(self.input_text.as_bytes())?;
                Ok(format!("{:X}", Output::from(ciphertext)))
            }
            OperationMode::Decrypt => {
                // Parse hex input for decryption
                let ciphertext = parse_hex(&self.input_text)?;
                let plaintext = cipher.decrypt(&ciphertext)?;
                let output = self.output_format.format(&Output::from(plaintext));
                Ok(output)
            }
        }
    }

    fn execute_ecb(&self, cipher: &dyn BlockCipher, text_bytes: &[u8]) -> CipherResult<String> {
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

/// Parses a hex string into bytes.
fn parse_hex(s: &str) -> CipherResult<Vec<u8>> {
    let trimmed = s.trim();
    let s = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);

    if !s.len().is_multiple_of(2) {
        return Err(CipherError::InvalidPadding(
            "hex string must have even length".into(),
        ));
    }

    (0..s.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&s[i..i + 2], 16)
                .map_err(|_| CipherError::InvalidPadding(format!("invalid hex at position {i}")))
        })
        .collect()
}
