use crate::key::{half28::Half28, key56::Key56};
use zeroize::ZeroizeOnDrop;

#[derive(ZeroizeOnDrop)]
pub struct CD56 {
    pub c: Half28,
    pub d: Half28,
}

impl CD56 {
    pub fn new(c: impl Into<Half28>, d: impl Into<Half28>) -> Self {
        Self {
            c: c.into(),
            d: d.into(),
        }
    }

    pub fn rotate_left(&mut self, amount: u8) {
        self.c = self.c.rotate_left(amount);
        self.d = self.d.rotate_left(amount);
    }
}

impl From<CD56> for Key56 {
    fn from(value: CD56) -> Self {
        Self::from_half28(&value.c, &value.d)
    }
}

impl From<&CD56> for Key56 {
    fn from(value: &CD56) -> Self {
        Self::from_half28(&value.c, &value.d)
    }
}
