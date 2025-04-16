use anchor_lang::prelude::*;

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

/// The struct containing parameters for initializing a stream
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
#[repr(C)]
pub struct CreateParams {
    /// Timestamp when the tokens start vesting
    pub start_time: u64,
    /// Deposited amount of tokens
    pub net_amount_deposited: u64,
    /// Time step (period) in seconds per which the vesting/release occurs
    pub period: u64,
    /// Amount released per period. Combined with `period`, we get a release rate.
    pub amount_per_period: u64,
    /// Vesting contract "cliff" timestamp
    pub cliff: u64,
    /// Amount unlocked at the "cliff" timestamp
    pub cliff_amount: u64,
    /// Whether a stream can be canceled by a sender
    pub cancelable_by_sender: bool,
    /// Whether a stream can be canceled by a recipient
    pub cancelable_by_recipient: bool,
    /// Whether a 3rd party can initiate withdraw in the name of recipient
    pub automatic_withdrawal: bool,
    /// Whether the sender can transfer the stream
    pub transferable_by_sender: bool,
    /// Whether the recipient can transfer the stream
    pub transferable_by_recipient: bool,
    /// Whether topup is enabled
    pub can_topup: bool,
    /// The name of this stream
    pub stream_name: [u8; 64],
    /// Withdraw frequency
    pub withdraw_frequency: u64,
    /// used as padding len in serialization in old streams, added for backwards compatibility
    pub ghost: u32,
    /// Whether the contract can be paused
    pub pausable: bool,
    /// Whether the contract can update release amount
    pub can_update_rate: bool,
}

/// Struct that represents Stream Contract stored on chain, this account **DOES NOT** have a discriminator.
/// 
/// May be read like so
/// 
/// ```rust
/// let stream_metadata: Contract = match try_from_slice_unchecked(&stream_data) {
///     Ok(v) => v,
///     Err(_) => return err!(ErrorCode::InvalidStreamMetadata),
/// };
/// ```
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
#[repr(C)]
pub struct Contract {
    /// Magic bytes
    pub magic: u64,
    /// Version of the program
    pub version: u8,
    /// Timestamp when stream was created
    pub created_at: u64,
    /// Amount of funds withdrawn
    pub amount_withdrawn: u64,
    /// Timestamp when stream was canceled (if canceled)
    pub canceled_at: u64,
    /// Timestamp at which stream can be safely canceled by a 3rd party
    /// (Stream is either fully vested or there isn't enough capital to
    /// keep it active)
    pub end_time: u64,
    /// Timestamp of the last withdrawal
    pub last_withdrawn_at: u64,
    /// Pubkey of the stream initializer
    pub sender: Pubkey,
    /// Pubkey of the stream initializer's token account
    pub sender_tokens: Pubkey,
    /// Pubkey of the stream recipient
    pub recipient: Pubkey,
    /// Pubkey of the stream recipient's token account
    pub recipient_tokens: Pubkey,
    /// Pubkey of the token mint
    pub mint: Pubkey,
    /// Escrow account holding the locked tokens for recipient
    pub escrow_tokens: Pubkey,
    /// Streamflow treasury authority
    pub streamflow_treasury: Pubkey,
    /// Escrow account holding the locked tokens for Streamflow (fee account)
    pub streamflow_treasury_tokens: Pubkey,
    /// The total fee amount for streamflow
    pub streamflow_fee_total: u64,
    /// The withdrawn fee amount for streamflow
    pub streamflow_fee_withdrawn: u64,
    /// Fee percentage for Streamflow
    pub streamflow_fee_percent: f32,
    /// Streamflow partner authority
    pub partner: Pubkey,
    /// Escrow account holding the locked tokens for Streamflow partner (fee account)
    pub partner_tokens: Pubkey,
    /// The total fee amount for the partner
    pub partner_fee_total: u64,
    /// The withdrawn fee amount for the partner
    pub partner_fee_withdrawn: u64,
    /// Fee percentage for partner
    pub partner_fee_percent: f32,
    /// The stream instruction
    pub ix: CreateParams,
    /// Padding for `ix: CreateParams` to allow for future upgrades.
    pub ix_padding: Vec<u8>,
    /// Stream is closed
    pub closed: bool,
    /// time of the current pause. 0 signifies unpaused state
    pub current_pause_start: u64,
    /// total time the contract was paused for
    pub pause_cumulative: u64,
    /// timestamp of last rate change for this stream.
    /// Rate can be changed with `update` instruction
    pub last_rate_change_time: u64,
    /// Accumulated unlocked tokens before last rate change (excluding cliff_amount)
    pub funds_unlocked_at_last_rate_change: u64,
}
