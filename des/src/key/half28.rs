use crate::key::secret_key;

secret_key! {
    /// 28-bit half (C or D), stored in lower 28 bits of u32.
    pub struct Half28(u32, 28, 0x0FFF_FFFF);
}

impl Half28 {
    #[must_use]
    pub const fn rotate_left(self, amount: u8) -> Self {
        let value = self.0;
        let main_shifted = (value << amount) & Self::MASK;
        let wrapped_bits = (value >> (28 - amount)) & ((1 << amount) - 1);
        Self::from_u32(main_shifted | wrapped_bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0x0F0C_CAAF, 0x0E19_955F, 1)] // C_1
    #[case(0x0E19_955F, 0x0C33_2ABF, 1)] // C_2
    #[case(0x0C33_2ABF, 0x00CC_AAFF, 2)] // C_3
    #[case(0x00CC_AAFF, 0x0332_ABFC, 2)] // C_4
    #[case(0x0332_ABFC, 0x0CCA_AFF0, 2)] // C_5
    #[case(0x0CCA_AFF0, 0x032A_BFC3, 2)] // C_6
    #[case(0x032A_BFC3, 0x0CAA_FF0C, 2)] // C_7
    #[case(0x0CAA_FF0C, 0x02AB_FC33, 2)] // C_8
    #[case(0x02AB_FC33, 0x0557_F866, 1)] // C_9
    #[case(0x0557_F866, 0x055F_E199, 2)] // C_10
    #[case(0x055F_E199, 0x057F_8665, 2)] // C_11
    #[case(0x057F_8665, 0x05FE_1995, 2)] // C_12
    #[case(0x05FE_1995, 0x07F8_6655, 2)] // C_13
    #[case(0x07F8_6655, 0x0FE1_9955, 2)] // C_14
    #[case(0x0FE1_9955, 0x0F86_6557, 2)] // C_15
    #[case(0x0F86_6557, 0x0F0C_CAAF, 1)] // C_16
    #[case(0x0556_678F, 0x0AAC_CF1E, 1)] // D_1
    #[case(0x0AAC_CF1E, 0x0559_9E3D, 1)] // D_2
    #[case(0x0559_9E3D, 0x0566_78F5, 2)] // D_3
    #[case(0x0566_78F5, 0x0599_E3D5, 2)] // D_4
    #[case(0x0599_E3D5, 0x0667_8F55, 2)] // D_5
    #[case(0x0667_8F55, 0x099E_3D55, 2)] // D_6
    #[case(0x099E_3D55, 0x0678_F556, 2)] // D_7
    #[case(0x0678_F556, 0x09E3_D559, 2)] // D_8
    #[case(0x09E3_D559, 0x03C7_AAB3, 1)] // D_9
    #[case(0x03C7_AAB3, 0x0F1E_AACC, 2)] // D_10
    #[case(0x0F1E_AACC, 0x0C7A_AB33, 2)] // D_11
    #[case(0x0C7A_AB33, 0x01EA_ACCF, 2)] // D_12
    #[case(0x01EA_ACCF, 0x07AA_B33C, 2)] // D_13
    #[case(0x07AA_B33C, 0x0EAA_CCF1, 2)] // D_14
    #[case(0x0EAA_CCF1, 0x0AAB_33C7, 2)] // D_15
    #[case(0x0AAB_33C7, 0x0556_678F, 1)] // D_16
    fn half28_rotation(#[case] key: u32, #[case] expected: u32, #[case] amount: u8) {
        let result = Half28::from_u32(key).rotate_left(amount).as_u32();

        assert_eq!(
            result, expected,
            "shift(0x{key:08X}, {amount}) should equal 0x{expected:08X}"
        );
    }
}
