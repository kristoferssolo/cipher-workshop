use crate::secret_key;

secret_key! {
    pub struct ExpandedKey(u32, 32, 0xFFFF_FFFF);
}
