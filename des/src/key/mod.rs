mod secret_int;
pub mod subkeys;

use crate::secret_int;

secret_int! {
    /// A single DES round subkey (48 bits stored in lower bits of u64).
    pub struct Subkey(u64, 48, 0x0000_FFFF_FFFF_FFFF);
}

secret_int! {
    /// 56-bit key after PC-1 (lower 56 bits used).
    pub struct Key56(u64, 56, 0x00FF_FFFF_FFFF_FFFF);
}

secret_int! {
    /// 28-bit half (C or D), stored in lower 28 bits of u32.
    pub struct Half28(u32, 28, 0x0FFF_FFFF);
}
