use crate::{
    block::{Block32, Block48, Block64, LR},
    constants::{E_BOX, FP, IP, P_BOX, S_BOXES},
    key::{Key, Subkey, Subkeys},
    utils::permutate,
};
use cipher_core::{BlockCipher, CipherAction, CipherError, CipherResult};

pub struct Des {
    subkeys: Subkeys,
}

impl Des {
    pub fn new(key: impl Into<Key>) -> CipherResult<Self> {
        let subkeys = Subkeys::from_key(&key.into())?;
        Ok(Self { subkeys })
    }
}

impl BlockCipher for Des {
    const BLOCK_SIZE: usize = 8;

    fn transform_impl(
        &self,
        block: &[u8],
        action: cipher_core::CipherAction,
    ) -> cipher_core::CipherResult<Vec<u8>> {
        let block_arr: [u8; Self::BLOCK_SIZE] = block
            .try_into()
            .map_err(|_| CipherError::invalid_block_size(Self::BLOCK_SIZE, block.len()))?;
        let block64 = Block64::from_be_bytes(block_arr);
        let permutated_block = ip(block64);

        let result = match action {
            CipherAction::Encrypt => feistel_rounds(permutated_block, self.subkeys.iter()),
            CipherAction::Decrypt => feistel_rounds(permutated_block, self.subkeys.iter_rev()),
        };

        let result = fp(result);

        Ok(result.into())
    }
}

#[inline]
#[must_use]
fn ip(block: Block64) -> Block64 {
    permutate(block.as_u64(), 64, 64, &IP).into()
}

#[must_use]
fn feistel_rounds<'a, I>(block: Block64, subkeys: I) -> Block64
where
    I: Iterator<Item = &'a Subkey>,
{
    let mut lr = LR::from(block);

    for subkey in subkeys {
        feistel(&mut lr, subkey);
    }
    lr.into()
}

fn feistel(lr: &mut LR, subkey: &Subkey) {
    let tmp = lr.right;
    lr.right = lr.left ^ f_function(lr.right, subkey);
    lr.left = tmp;
}

#[must_use]
fn f_function(right: Block32, subkey: &Subkey) -> Block32 {
    let expanded = expansion_permutation(right);
    let xored = expanded ^ subkey;
    let sboxed = s_box_substitution(xored);
    p_box_permutation(sboxed)
}

#[inline]
#[must_use]
fn expansion_permutation(right: Block32) -> Block48 {
    permutate(right.as_u64(), 32, 48, &E_BOX).into()
}

#[must_use]
fn s_box_substitution(block: Block48) -> Block32 {
    let six_bit_blocks = block.as_block6_array();
    S_BOXES
        .iter()
        .zip(six_bit_blocks.iter())
        .enumerate()
        .fold(0, |acc, (idx, (s_box, block6))| {
            let row = block6.to_row();
            let col = block6.to_col();

            let sbox_value = s_box[row][col];
            let shift_amount = (7 - idx) * 4;

            acc | (u32::from(sbox_value) << shift_amount)
        })
        .into()
}

#[inline]
#[must_use]
fn p_box_permutation(block: Block32) -> Block32 {
    permutate(block.as_u64(), 32, 32, &P_BOX).into()
}

#[inline]
#[must_use]
fn fp(result: Block64) -> Block64 {
    permutate(result.as_u64(), 64, 64, &FP).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TEST_PLAINTEXT: u64 = 0x0123_4567_89AB_CDEF;

    #[rstest]
    #[case(TEST_PLAINTEXT, 0xCC00_CCFF_F0AA_F0AA)]
    fn initial_permutation(#[case] input: u64, #[case] expected: u64) {
        let result = ip(input.into()).as_u64();
        assert_eq!(
            result, expected,
            "Initial permutation failed. Expected 0x{expected:016X}, got 0x{result:016X}"
        );
    }

    #[rstest]
    #[case(0xF0AA_F0AA, 0x7A15_557A_1555)] // Round 1
    #[case(0xEF4A_6544, 0x75EA_5430_AA09)] // Round 2
    #[case(0xCC01_7709, 0xE580_02BA_E853)] // Round 3
    #[case(0xA25C_0BF4, 0x5042_F805_7FA9)] // Round 4
    #[case(0x7722_0045, 0xBAE9_0400_020A)] // Round 5
    #[case(0x8A4F_A637, 0xC542_5FD0_C1AF)] // Round 6
    #[case(0xE967_CD69, 0xF52B_0FE5_AB53)] // Round 7
    #[case(0x064A_BA10, 0x00C2_555F_40A0)] // Round 8
    #[case(0xD569_4B90, 0x6AAB_52A5_7CA1)] // Round 9
    #[case(0x247C_C67A, 0x1083_F960_C3F4)] // Round 10
    #[case(0xB7D5_D7B2, 0x5AFE_ABEA_FDA5)] // Round 11
    #[case(0xC578_3C78, 0x60AB_F01F_83F1)] // Round 12
    #[case(0x75BD_1858, 0x3ABD_FA8F_02F0)] // Round 13
    #[case(0x18C3_155A, 0x0F16_068A_AAF4)] // Round 14
    #[case(0xC28C_960D, 0xE054_594A_C05B)] // Round 15
    #[case(0x4342_3234, 0x206A_041A_41A8)] // Round 16
    fn permutation_expansion(#[case] block: u32, #[case] expected: u64) {
        let result = expansion_permutation(block.into()).as_u64();
        assert_eq!(
            result, expected,
            "Expansion permutaion failed. Expected {expected:016X}, got {result:016X}"
        );
    }

    #[rstest]
    #[case(0x6117_BA86_6527, 0x5C82_B597)] // Round 1
    #[case(0x0C44_8DEB_63EC, 0xF8D0_3AAE)] // Round 2
    #[case(0xB07C_88F8_27CA, 0x2710_E16F)] // Round 3
    #[case(0x22EF_2EDE_4AB4, 0x21ED_9F3A)] // Round 4
    #[case(0xC605_03EB_51A2, 0x50C8_31EB)] // Round 5
    #[case(0xA6E7_6180_BA80, 0x41F3_4C3D)] // Round 6
    #[case(0x19AF_B813_B3EF, 0x1075_40AD)] // Round 7
    #[case(0xF748_6F9E_7B5B, 0x6C18_7CAE)] // Round 8
    #[case(0x8A70_B948_9B20, 0x110C_5777)] // Round 9
    #[case(0xA170_BEDA_85BB, 0xDA04_5275)] // Round 10
    #[case(0x7BA1_7834_2E23, 0x7305_D101)] // Round 11
    #[case(0x15DA_058B_E418, 0x7B8B_2635)] // Round 12
    #[case(0xAD78_2B75_B8B1, 0x9AD1_8B4F)] // Round 13
    #[case(0x5055_B178_4DCE, 0x6479_9AF1)] // Round 14
    #[case(0x5FC5_D477_FF51, 0xB2E8_8D3C)] // Round 15
    #[case(0xEB57_8F14_565D, 0xA783_2429)] // Round 16
    fn sbox_subsitution(#[case] block: u64, #[case] expected: u32) {
        let result = s_box_substitution(block.into()).as_u32();
        assert_eq!(
            result, expected,
            "S-BOX substituion failed. Expected {expected:08X}, got {result:08X}"
        );
    }

    #[rstest]
    #[case(0x5C82_B597, 0x234A_A9BB)] // Round 1
    #[case(0xF8D0_3AAE, 0x3CAB_87A3)] // Round 2
    #[case(0x2710_E16F, 0x4D16_6EB0)] // Round 3
    #[case(0x21ED_9F3A, 0xBB23_774C)] // Round 4
    #[case(0x50C8_31EB, 0x2813_ADC3)] // Round 5
    #[case(0x41F3_4C3D, 0x9E45_CD2C)] // Round 6
    #[case(0x1075_40AD, 0x8C05_1C27)] // Round 7
    #[case(0x6C18_7CAE, 0x3C0E_86F9)] // Round 8
    #[case(0x110C_5777, 0x2236_7C6A)] // Round 9
    #[case(0xDA04_5275, 0x62BC_9C22)] // Round 10
    #[case(0x7305_D101, 0xE104_FA02)] // Round 11
    #[case(0x7B8B_2635, 0xC268_CFEA)] // Round 12
    #[case(0x9AD1_8B4F, 0xDDBB_2922)] // Round 13
    #[case(0x6479_9AF1, 0xB731_8E55)] // Round 14
    #[case(0xB2E8_8D3C, 0x5B81_276E)] // Round 15
    #[case(0xA783_2429, 0xC8C0_4F98)] // Round 16
    fn permuation_pbox(#[case] block: u32, #[case] expected: u32) {
        let result = p_box_permutation(block.into()).as_u32();
        assert_eq!(
            result, expected,
            "P-BOX permutation failed. Expected {expected:08X}, got {result:08X}"
        );
    }

    #[rstest]
    #[case(0x0A4C_D995_4342_3234, 0x85E8_1354_0F0A_B405)]
    fn final_permutation(#[case] input: u64, #[case] expected: u64) {
        let result = fp(input.into()).as_u64();
        assert_eq!(
            result, expected,
            "Final permutation failed. Expected 0x{expected:016X}, got 0x{result:016X}"
        );
    }
}
