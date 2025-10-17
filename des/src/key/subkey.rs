use crate::key::secret_key;

secret_key! {
    /// A single DES round subkey (48 bits stored in lower bits of u64).
    pub struct Subkey(u64, 48, 0x0000_FFFF_FFFF_FFFF);
}

impl Subkey {
    /// Zero value.
    pub const fn zero() -> Self {
        Self(0)
    }
}
