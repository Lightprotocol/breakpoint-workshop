use anchor_lang::prelude::*;
use groth16_solana::groth16::Groth16Verifyingkey;

pub const VERIFYINGKEY_SWAPS: Groth16Verifyingkey = Groth16Verifyingkey {
    nr_pubinputs: 2,
    vk_alpha_g1: [
        45, 77, 154, 167, 227, 2, 217, 223, 65, 116, 157, 85, 7, 148, 157, 5, 219, 234, 51, 251,
        177, 108, 100, 59, 34, 245, 153, 162, 190, 109, 242, 226, 20, 190, 221, 80, 60, 55, 206,
        176, 97, 216, 236, 96, 32, 159, 227, 69, 206, 137, 131, 10, 25, 35, 3, 1, 240, 118, 202,
        255, 0, 77, 25, 38,
    ],

    vk_beta_g2: [
        9, 103, 3, 47, 203, 247, 118, 209, 175, 201, 133, 248, 136, 119, 241, 130, 211, 132, 128,
        166, 83, 242, 222, 202, 169, 121, 76, 188, 59, 243, 6, 12, 14, 24, 120, 71, 173, 76, 121,
        131, 116, 208, 214, 115, 43, 245, 1, 132, 125, 214, 139, 192, 224, 113, 36, 30, 2, 19, 188,
        127, 193, 61, 183, 171, 48, 76, 251, 209, 224, 138, 112, 74, 153, 245, 232, 71, 217, 63,
        140, 60, 170, 253, 222, 196, 107, 122, 13, 55, 157, 166, 154, 77, 17, 35, 70, 167, 23, 57,
        193, 177, 164, 87, 168, 199, 49, 49, 35, 210, 77, 47, 145, 146, 248, 150, 183, 198, 62,
        234, 5, 169, 213, 127, 6, 84, 122, 208, 206, 200,
    ],

    vk_gamme_g2: [
        25, 142, 147, 147, 146, 13, 72, 58, 114, 96, 191, 183, 49, 251, 93, 37, 241, 170, 73, 51,
        53, 169, 231, 18, 151, 228, 133, 183, 174, 243, 18, 194, 24, 0, 222, 239, 18, 31, 30, 118,
        66, 106, 0, 102, 94, 92, 68, 121, 103, 67, 34, 212, 247, 94, 218, 221, 70, 222, 189, 92,
        217, 146, 246, 237, 9, 6, 137, 208, 88, 95, 240, 117, 236, 158, 153, 173, 105, 12, 51, 149,
        188, 75, 49, 51, 112, 179, 142, 243, 85, 172, 218, 220, 209, 34, 151, 91, 18, 200, 94, 165,
        219, 140, 109, 235, 74, 171, 113, 128, 141, 203, 64, 143, 227, 209, 231, 105, 12, 67, 211,
        123, 76, 230, 204, 1, 102, 250, 125, 170,
    ],

    vk_delta_g2: [
        8, 239, 8, 166, 122, 74, 18, 202, 105, 184, 57, 112, 83, 167, 55, 119, 194, 98, 196, 175,
        116, 179, 61, 4, 236, 247, 200, 86, 209, 45, 210, 201, 19, 68, 58, 83, 95, 202, 70, 145,
        220, 223, 160, 26, 134, 156, 248, 83, 58, 150, 140, 160, 1, 217, 212, 207, 37, 89, 238, 78,
        56, 138, 198, 166, 25, 244, 97, 112, 10, 220, 246, 0, 214, 241, 242, 116, 254, 253, 13, 61,
        253, 171, 139, 126, 224, 76, 245, 42, 34, 141, 13, 242, 157, 79, 92, 209, 24, 30, 206, 163,
        248, 236, 125, 163, 31, 173, 244, 235, 76, 77, 174, 231, 240, 114, 9, 174, 74, 41, 202,
        231, 43, 128, 137, 54, 112, 98, 12, 156,
    ],

    vk_ic: &[
        [
            17, 199, 215, 154, 254, 234, 0, 30, 77, 46, 186, 133, 60, 200, 190, 73, 212, 136, 107,
            126, 33, 237, 136, 229, 98, 156, 162, 108, 107, 195, 97, 4, 28, 123, 159, 221, 97, 146,
            218, 60, 106, 81, 230, 93, 3, 214, 242, 143, 184, 182, 106, 240, 44, 172, 228, 169,
            137, 182, 163, 130, 19, 226, 228, 35,
        ],
        [
            1, 13, 10, 61, 197, 88, 187, 179, 183, 117, 195, 99, 119, 222, 103, 200, 117, 21, 16,
            235, 96, 89, 34, 238, 93, 220, 109, 172, 141, 90, 221, 191, 32, 33, 169, 133, 63, 164,
            38, 134, 147, 77, 48, 227, 127, 27, 155, 142, 133, 46, 182, 124, 255, 224, 25, 158, 60,
            240, 242, 223, 48, 139, 145, 23,
        ],
        [
            45, 201, 182, 185, 123, 34, 214, 148, 128, 61, 193, 244, 148, 189, 188, 26, 133, 129,
            130, 11, 82, 75, 40, 162, 216, 98, 165, 8, 154, 61, 232, 223, 6, 163, 18, 201, 226, 53,
            95, 223, 156, 66, 213, 30, 146, 71, 83, 123, 104, 38, 160, 223, 122, 40, 229, 31, 11,
            186, 21, 4, 145, 137, 132, 76,
        ],
    ],
};
#[account]
pub struct ZKswapsProofInputs {
    public_app_verifier: u8,
    transaction_hash: u8,
    tx_integrity_hash: u8,
    in_amount: [[u8; 2]; 4],
    in_public_key: [u8; 4],
    in_blinding: [u8; 4],
    in_app_data_hash: [u8; 4],
    in_pool_type: [u8; 4],
    in_verifier_pubkey: [u8; 4],
    in_indices: [[[u8; 3]; 2]; 4],
    output_commitment: [u8; 4],
    out_amount: [[u8; 2]; 4],
    out_pubkey: [u8; 4],
    out_blinding: [u8; 4],
    out_app_data_hash: [u8; 4],
    out_indices: [[[u8; 3]; 2]; 4],
    out_pool_type: [u8; 4],
    out_verifier_pubkey: [u8; 4],
    asset_pubkeys: [u8; 3],
    transaction_version: u8,
    is_in_app_utxo_offer_utxo: [u8; 4],
    offer_utxo_price_sol: u8,
    offer_utxo_price_spl: u8,
    offer_utxo_spl_asset: u8,
    offer_utxo_recipient: u8,
    offer_utxo_recipient_encryption_public_key: u8,
    offer_utxo_public_key: u8,
    offer_utxo_blinding: u8,
    offer_utxo_psp_owner: u8,
    offer_utxo_amount_sol: u8,
    offer_utxo_amount_spl: u8,
    offer_utxo_asset_spl: u8,
    offer_utxo_tx_version: u8,
    offer_utxo_pool_type: u8,
    take_offer_instruction: u8,
    is_out_app_utxo_offer_reward_utxo: [u8; 4],
    offer_reward_utxo_public_key: u8,
    offer_reward_utxo_blinding: u8,
    offer_reward_utxo_psp_owner: u8,
    offer_reward_utxo_amount_sol: u8,
    offer_reward_utxo_amount_spl: u8,
    offer_reward_utxo_asset_spl: u8,
    offer_reward_utxo_tx_version: u8,
    offer_reward_utxo_pool_type: u8,
}
#[account]
pub struct ZKswapsPublicInputs {
    public_app_verifier: u8,
    transaction_hash: u8,
}
#[account]
pub struct InstructionDataLightInstructionSwapsSecond {
    public_app_verifier: [u8; 32],
    transaction_hash: [u8; 32],
}
