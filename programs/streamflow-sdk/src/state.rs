use anchor_lang::solana_program::{
    pubkey::Pubkey,
};

/// Streamflow Treasury address, by default receives 0.25% of tokens deposited
pub const STRM_TREASURY: &str = "5SEpbdjFK5FxwTvfsGMXVQTD2v4M2c5tyRTxhdsPkgDw";
/// Streamflow Withdrawor address, this account will process withdrawals
pub const WITHDRAWOR_ADDRESS: &str = "wdrwhnCv4pzW8beKsbPa4S2UDZrXenjg16KJdKSpb5u";
/// Address of Fee Oracle that stores information about fees for speficic partners
pub const FEE_ORACLE_ADDRESS: &str = "B743wFVk2pCYhV91cn287e1xY7f1vt4gdY48hhNiuQmT";

/// Prefix used to derive Escrow account address
pub const ESCROW_SEED_PREFIX: &[u8] = b"strm";
/// Size of Stream metadata
pub const METADATA_LEN: usize = 1104;

/// You can also use id that sdk exposes like so streamflow_sdk::id()
pub const STREAMFLOW_PROGRAM_ID: &str = "strmRqUCoQUgGUan5YhzUZa6KqdzwX5L6FpUxfmKg5m";
pub const STREAMFLOW_DEVNET_PROGRAM_ID: &str = "HqDGZjaVRXJ9MGRQEw7qDc2rAr6iH1n1kAQdCZaCMfMZ";

pub fn find_escrow_account(seed: &[u8], pid: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[ESCROW_SEED_PREFIX, seed], pid)
}
