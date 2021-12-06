use core::str::from_utf8;
use nanos_sdk::bindings::*;
use nanos_sdk::ecc;
use nanos_sdk::ecc::{CurvesId, Ed25519Signature};
use nanos_sdk::io::SyscallError;
use nanos_ui::ui;
use crate::utils::to_hex;

// pub const BIP32_PATH: [u32; 5] = nanos_sdk::ecc::make_bip32_path(b"m/44'/535348'/0'/0/0");
pub const BIP32_PATH: [u32; 5] = [0x8000002Cu32; 5]; // nanos_sdk::ecc::make_bip32_path(b"m/44'/1234'/0'/0'/0'/");

/// Helper function that derives the seed over secp256k1
pub fn bip32_derive_ed25519(path: &[u32]) -> Result<[u8; 32], SyscallError> {
    // ui::popup("bip321");

    let mut raw_key = [0u8; 32];
    // ui::popup("bip322");
    nanos_sdk::ecc::bip32_derive_with_seed_key(CurvesId::Ed25519, path, &mut raw_key)?;
    // ui::popup("bip323");
    // let hex0 = to_hex(&raw_key).unwrap();
    // let m = from_utf8(&hex0).unwrap();
    // ui::MessageScroller::new(m).event_loop();
    Ok(raw_key)
}

// /// Helper function that signs with ECDSA in deterministic nonce,
// /// using SHA256
// pub fn detecdsa_sign(
//     m: &[u8],
//     ec_k: &cx_ecfp_private_key_t,
// ) -> Option<(DerEncodedEcdsaSignature, u32)> {
//     nanos_sdk::ecc::ecdsa_sign(ec_k, CX_RND_RFC6979 | CX_LAST, CX_SHA256, m)
// }

pub fn ed25519_sign(
    m: &[u8],
    ec_k: &cx_ecfp_private_key_t,
) -> Result<Ed25519Signature, u32> {
    // TODO cx_eddsa_sign_no_throw
    // See lib_cxng/include/lcx_eddsa.h
    nanos_sdk::ecc::ed25519_sign(ec_k, CX_SHA512, m)
}

pub type Ed25519PubKey = [u8; 32];
pub fn get_pubkey() -> Result<Ed25519PubKey, SyscallError> {
    // Calculate and show test sha3_256 hash
    let sha_input = [0u8; 0];
    let sha_result =  ecc::sha3(&sha_input);
    let hex0 = to_hex(&sha_result).unwrap();
    let sha_hex = from_utf8(&hex0).unwrap();
    ui::popup("sha3");
    ui::MessageScroller::new(sha_hex).event_loop();
    // ui::popup("pubkey11");
    let raw_key = bip32_derive_ed25519(&BIP32_PATH)?;
    // ui::popup("pubkey2");
    let mut ec_k = nanos_sdk::ecc::ec_init_key(CurvesId::Ed25519, &raw_key)?;
    // ui::popup("pubkey3");
    let pk = nanos_sdk::ecc::ec_get_pubkey(CurvesId::Ed25519, &mut ec_k)?;
    // LedgerHQ/app-solana/src/utils.c void getPublicKey(..)
    let mut pub_key_res = [0u8; 32];
    for i in 0..pub_key_res.len() {
        pub_key_res[i] = pk.W[64 - i];
    }
    if pk.W[32] & 1 != 0 {
        pub_key_res[31] |= 0x80;
    }
    Ok(pub_key_res)

}

pub fn get_private_key() -> Result<nanos_sdk::bindings::cx_ecfp_private_key_t, SyscallError> {
    let raw_key = bip32_derive_ed25519(&BIP32_PATH)?;
    nanos_sdk::ecc::ec_init_key(CurvesId::Ed25519, &raw_key)
}
