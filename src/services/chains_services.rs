use ethers::{core::types::Address, signers::{LocalWallet, Signer}};
use hex;

pub fn generate_chain_data(secret_key: &str) -> (String, String) {
    let wallet: LocalWallet = secret_key.parse().unwrap();
    let address = wallet.address();
    let address_str = format!("{:#x}", address);
    let uncompressed_public_key = wallet.signer().verifying_key().to_encoded_point(false);
    let public_key = hex::encode(uncompressed_public_key.as_bytes());
    (address_str, public_key)
}