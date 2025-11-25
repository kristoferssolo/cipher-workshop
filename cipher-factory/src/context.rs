use crate::{Algorithm, OperationChoice, OutputFormat};
use cipher_core::{BlockCipher, CipherResult};

pub struct CipherContext {
    pub algorithm: Algorithm,
    pub operation: OperationChoice,
    pub key: String,
    pub input_text: String,
    pub output_format: OutputFormat,
}

impl CipherContext {
    pub fn process(&self) -> CipherResult<String> {
        let text_bytes = self.algorithm.parse_text(&self.input_text)?;
        let cipher = self.algorithm.new_cipher(&self.key)?;
        self.execute(cipher.as_ref(), &text_bytes)
    }

    fn execute(&self, cipher: &dyn BlockCipher, text_bytes: &[u8]) -> CipherResult<String> {
        match self.operation {
            OperationChoice::Encrypt => {
                let ciphertext = cipher.encrypt(text_bytes)?;
                Ok(format!("{ciphertext:X}"))
            }
            OperationChoice::Decrypt => {
                let plaintext = cipher.decrypt(text_bytes)?;
                let output = self.output_format.to_string(&plaintext);
                Ok(output)
            }
        }
    }
}
