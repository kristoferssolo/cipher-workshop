use crate::key::secret_int;

secret_int! {
    /// A single DES round subkey (48 bits stored in lower bits of u64).
    pub struct Subkey(u64, 48, 0x0000_FFFF_FFFF_FFFF);
}
