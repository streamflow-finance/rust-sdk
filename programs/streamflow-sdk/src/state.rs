use anchor_lang::prelude::*;

/// Streamflow Treasury address, by default receives 0.25% of tokens deposited
pub const STRM_TREASURY: &str = "5SEpbdjFK5FxwTvfsGMXVQTD2v4M2c5tyRTxhdsPkgDw";
/// Streamflow Withdrawor address, this account will process withdrawals
pub const WITHDRAWOR_ADDRESS: &str = "wdrwhnCv4pzW8beKsbPa4S2UDZrXenjg16KJdKSpb5u";
/// Address of Fee Oracle that stores information about fees for speficic partners
pub const FEE_ORACLE_ADDRESS: &str = "B743wFVk2pCYhV91cn287e1xY7f1vt4gdY48hhNiuQmT";

/// Prefix used to derive Escrow account address
pub const ESCROW_SEED_PREFIX: &[u8] = b"strm";
/// Prefix used to derive Metadata PDA address (v2)
pub const METADATA_SEED_PREFIX: &[u8] = b"strm-met";
/// Fixed padding to account for additional fields added to CreateParams
pub const CREATE_PARAMS_PADDING: usize = 121;
/// Size of Stream metadata
pub const METADATA_LEN: usize = 1104;

/// You can also use id that sdk exposes like so streamflow_sdk::id()
pub const STREAMFLOW_PROGRAM_ID: &str = "strmRqUCoQUgGUan5YhzUZa6KqdzwX5L6FpUxfmKg5m";
pub const STREAMFLOW_DEVNET_PROGRAM_ID: &str = "HqDGZjaVRXJ9MGRQEw7qDc2rAr6iH1n1kAQdCZaCMfMZ";

pub fn find_escrow_account(seed: &[u8], pid: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[ESCROW_SEED_PREFIX, seed], pid)
}

/// Derive metadata PDA address for create_v2 instructions.
///
/// Seeds: `["strm-met", mint, payer, nonce_be_bytes]`
///
/// For `create_v2`, `payer` is the sender.
/// For `create_unchecked_with_payer_v2`, `payer` is the payer account.
pub fn derive_metadata(mint: &Pubkey, payer: &Pubkey, nonce: u32, pid: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[METADATA_SEED_PREFIX, mint.as_ref(), payer.as_ref(), nonce.to_be_bytes().as_ref()],
        pid,
    )
}

/// Calculate fee amount from a provided amount
pub fn calculate_fee_from_amount(amount: u64, percentage: f32) -> u64 {
    if percentage <= 0.0 {
        return 0;
    }
    let precision_factor: f32 = 1000000.0;
    // largest it can get is MAX_FEE * 10^4
    let factor = (percentage / 100.0 * precision_factor) as u128;

    // this does not fit if amount  itself cannot fit into u64
    (amount as u128 * factor / precision_factor as u128) as u64
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
    /// used as padding len in serialization, added for backwards compatibility
    pub ghost2: u32,
    /// Whether the contract metadata is a PDA (v2)
    pub is_pda: bool,
    /// Nonce for PDA-based metadata derivation (v2)
    pub nonce: u32,
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
    pub ix_padding: [u8; CREATE_PARAMS_PADDING],
    /// Whether Stream is closed
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
    /// Creation SOL fee
    pub creation_fee: u32,
    /// Whether SOL fees have been claimed from the metadata account
    pub creation_fee_claimed: bool,
    /// Auto-claim SOL fee
    pub auto_claim_fee: u32,
    /// Whether SOL fees have been claimed from the metadata account
    pub auto_claim_fee_claimed: bool,
    /// Pubkey of the old (drained) metadata account, used for escrow PDA derivation.
    /// Zero (default) means this IS the original metadata -- escrow derives from self.
    pub old_metadata: Pubkey,
    /// Wallet that paid for the Contract creation, may be used for PDA derivation
    pub payer: Pubkey,
    /// Bump used for the Contract Metadata if it's a PDA
    pub bump: u8,
}

impl Contract {
    pub fn start_time(&self) -> u64 {
        if self.ix.cliff > 0 {
            self.ix.cliff
        } else {
            self.ix.start_time
        }
    }

    pub fn effective_start_time(&self) -> u64 {
        std::cmp::max(self.last_rate_change_time, self.start_time())
    }

    pub fn pause_time(&self, now: u64) -> u64 {
        if self.current_pause_start > 0 {
            return self.pause_cumulative + now - self.current_pause_start;
        }
        self.pause_cumulative
    }

    /// Calculate time of the last unlock, no partial unlocks are allowed
    pub fn last_unlock_time(&self, now: u64) -> u64 {
        let pause_time = self.pause_time(now);
        let effective_stream_start = self.effective_start_time();
        let stream_duration = now - effective_stream_start - pause_time;
        let periods_passed = stream_duration / self.ix.period;

        effective_stream_start + periods_passed * self.ix.period
    }

    /// amount available that is vested (excluding cliff)
    pub fn vested_available(&self, now: u64) -> u64 {
        let start = self.start_time();
        // if pause started before start/cliff and is still active, no unlocks
        if self.current_pause_start < start && self.current_pause_start != 0 {
            return 0;
        }
        // available from streaming based on current rate
        let effective_stream_duration = now - self.effective_start_time() - self.pause_time(now);
        let effective_periods_passed = effective_stream_duration / self.ix.period;
        let effective_amount_available = effective_periods_passed * self.ix.amount_per_period;

        effective_amount_available + self.funds_unlocked_at_last_rate_change
    }

    pub fn available_to_claim(&self, now: u64, fee_percentage: f32) -> u64 {
        if self.start_time() > now
            || self.ix.net_amount_deposited == 0
            || self.ix.net_amount_deposited == self.amount_withdrawn
        {
            return 0;
        }
        if now >= self.end_time && self.current_pause_start == 0 {
            return self.ix.net_amount_deposited - self.amount_withdrawn;
        }

        let vested_available =
            calculate_fee_from_amount(self.vested_available(now), fee_percentage);
        let cliff_available = calculate_fee_from_amount(self.cliff_available(now), fee_percentage);
        let sum_available = vested_available + cliff_available;
        sum_available - self.amount_withdrawn
    }

    pub fn cliff_available(&self, now: u64) -> u64 {
        if self.current_pause_start < self.ix.cliff && self.current_pause_start != 0 {
            return 0;
        }
        if now < self.ix.cliff {
            return 0;
        }
        self.ix.cliff_amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helpers for building a `Contract` and `CreateParams` with the fields that
    // affect account-transition math. Anything not set here is irrelevant to the
    // helpers under test, so we leave it as zero / default.

    fn default_params() -> CreateParams {
        CreateParams {
            start_time: 0,
            net_amount_deposited: 0,
            period: 1,
            amount_per_period: 0,
            cliff: 0,
            cliff_amount: 0,
            cancelable_by_sender: false,
            cancelable_by_recipient: false,
            automatic_withdrawal: false,
            transferable_by_sender: false,
            transferable_by_recipient: false,
            can_topup: false,
            stream_name: [0u8; 64],
            withdraw_frequency: 0,
            ghost: 0,
            pausable: false,
            can_update_rate: false,
            ghost2: 0,
            is_pda: false,
            nonce: 0,
        }
    }

    fn contract_with(ix: CreateParams) -> Contract {
        Contract {
            magic: 0,
            version: 1,
            created_at: 0,
            amount_withdrawn: 0,
            canceled_at: 0,
            end_time: 0,
            last_withdrawn_at: 0,
            sender: Pubkey::default(),
            sender_tokens: Pubkey::default(),
            recipient: Pubkey::default(),
            recipient_tokens: Pubkey::default(),
            mint: Pubkey::default(),
            escrow_tokens: Pubkey::default(),
            streamflow_treasury: Pubkey::default(),
            streamflow_treasury_tokens: Pubkey::default(),
            streamflow_fee_total: 0,
            streamflow_fee_withdrawn: 0,
            streamflow_fee_percent: 0.0,
            partner: Pubkey::default(),
            partner_tokens: Pubkey::default(),
            partner_fee_total: 0,
            partner_fee_withdrawn: 0,
            partner_fee_percent: 0.0,
            ix,
            ix_padding: [0u8; CREATE_PARAMS_PADDING],
            closed: false,
            current_pause_start: 0,
            pause_cumulative: 0,
            last_rate_change_time: 0,
            funds_unlocked_at_last_rate_change: 0,
            creation_fee: 0,
            creation_fee_claimed: false,
            auto_claim_fee: 0,
            auto_claim_fee_claimed: false,
            old_metadata: Pubkey::default(),
            payer: Pubkey::default(),
            bump: 0,
        }
    }

    // ---- start_time / effective_start_time ----

    #[test]
    fn start_time_returns_cliff_when_cliff_is_set() {
        let mut ix = default_params();
        ix.start_time = 100;
        ix.cliff = 250;
        let c = contract_with(ix);
        assert_eq!(c.start_time(), 250);
    }

    #[test]
    fn start_time_falls_back_to_start_when_cliff_is_zero() {
        let mut ix = default_params();
        ix.start_time = 100;
        ix.cliff = 0;
        let c = contract_with(ix);
        assert_eq!(c.start_time(), 100);
    }

    #[test]
    fn effective_start_time_uses_max_of_rate_change_and_start() {
        // rate change before start_time() -> effective is start_time()
        let mut ix = default_params();
        ix.start_time = 1_000;
        ix.cliff = 0;
        let mut c = contract_with(ix);
        c.last_rate_change_time = 500;
        assert_eq!(c.effective_start_time(), 1_000);

        // rate change after start_time() -> effective is the rate change
        c.last_rate_change_time = 1_500;
        assert_eq!(c.effective_start_time(), 1_500);

        // cliff takes precedence for start_time() but rate change can still push it out
        let mut ix2 = default_params();
        ix2.start_time = 1_000;
        ix2.cliff = 800;
        let mut c2 = contract_with(ix2);
        c2.last_rate_change_time = 900;
        assert_eq!(c2.start_time(), 800);
        assert_eq!(c2.effective_start_time(), 900);
    }

    // ---- pause_time ----

    #[test]
    fn pause_time_returns_cumulative_when_not_paused() {
        let mut c = contract_with(default_params());
        c.pause_cumulative = 42;
        c.current_pause_start = 0;
        assert_eq!(c.pause_time(1_000_000), 42);
    }

    #[test]
    fn pause_time_includes_open_pause_window() {
        let mut c = contract_with(default_params());
        c.pause_cumulative = 10;
        c.current_pause_start = 100; // paused at t=100
        // now=250 -> open pause has lasted 150s, total = 10 + 150 = 160
        assert_eq!(c.pause_time(250), 160);
    }

    // ---- last_unlock_time ----

    #[test]
    fn last_unlock_time_advances_in_full_periods() {
        let mut ix = default_params();
        ix.start_time = 1_000;
        ix.period = 60;
        ix.amount_per_period = 1;
        let c = contract_with(ix);
        // (2500 - 1000) = 1500s; 1500 / 60 = 25 periods; last unlock at 1000 + 25*60 = 2500
        assert_eq!(c.last_unlock_time(2_500), 2_500);
        // 25 periods + 1s -> still 2500 (no partial unlock)
        assert_eq!(c.last_unlock_time(2_501), 2_500);
        // 26 full periods -> 1000 + 26*60 = 2560
        assert_eq!(c.last_unlock_time(2_560), 2_560);
    }

    #[test]
    fn last_unlock_time_respects_pause_cumulative() {
        let mut ix = default_params();
        ix.start_time = 1_000;
        ix.period = 60;
        let mut c = contract_with(ix);
        c.pause_cumulative = 30;
        // effective stream duration = 1600 - 1000 - 30 = 570; 570/60 = 9; 1000 + 9*60 = 1540
        assert_eq!(c.last_unlock_time(1_600), 1_540);
    }

    // ---- vested_available ----

    #[test]
    fn vested_available_zero_when_pause_started_before_start_and_still_active() {
        let mut ix = default_params();
        ix.start_time = 1_000;
        ix.period = 60;
        ix.amount_per_period = 10;
        let mut c = contract_with(ix);
        c.current_pause_start = 500; // paused at 500, before start_time
        // pause started before start AND still active (current_pause_start != 0)
        assert_eq!(c.vested_available(2_000), 0);
    }

    #[test]
    fn vested_available_uses_rate_change_baseline() {
        let mut ix = default_params();
        ix.start_time = 1_000;
        ix.period = 100;
        ix.amount_per_period = 5;
        let mut c = contract_with(ix);
        // at t=2000 (without rate change): (2000-1000)/100 * 5 = 50
        assert_eq!(c.vested_available(2_000), 50);
        // rate change at t=1500 adds a baseline of 25 already-unlocked tokens
        c.last_rate_change_time = 1_500;
        c.funds_unlocked_at_last_rate_change = 25;
        // effective start = 1500, effective duration = 2000-1500 = 500, periods = 5, +25 baseline = 50
        assert_eq!(c.vested_available(2_000), 50);
    }

    // ---- cliff_available ----

    #[test]
    fn cliff_available_zero_before_cliff() {
        let mut ix = default_params();
        ix.cliff = 1_000;
        ix.cliff_amount = 123;
        let c = contract_with(ix);
        assert_eq!(c.cliff_available(999), 0);
        assert_eq!(c.cliff_available(0), 0);
    }

    #[test]
    fn cliff_available_returns_cliff_amount_at_and_after_cliff() {
        let mut ix = default_params();
        ix.cliff = 1_000;
        ix.cliff_amount = 123;
        let c = contract_with(ix);
        assert_eq!(c.cliff_available(1_000), 123);
        assert_eq!(c.cliff_available(2_000), 123);
    }

    #[test]
    fn cliff_available_zero_if_pause_started_before_cliff_and_still_active() {
        let mut ix = default_params();
        ix.cliff = 1_000;
        ix.cliff_amount = 123;
        let mut c = contract_with(ix);
        c.current_pause_start = 500;
        // pause started before cliff and still active
        assert_eq!(c.cliff_available(2_000), 0);
    }

    // ---- available_to_claim ----

    #[test]
    fn available_to_claim_zero_before_start() {
        let mut ix = default_params();
        ix.start_time = 1_000;
        ix.net_amount_deposited = 1_000;
        ix.period = 100;
        ix.amount_per_period = 10;
        let c = contract_with(ix);
        assert_eq!(c.available_to_claim(500, 0.0), 0);
    }

    #[test]
    fn available_to_claim_zero_when_already_fully_withdrawn() {
        let mut ix = default_params();
        ix.start_time = 1_000;
        ix.net_amount_deposited = 1_000;
        ix.period = 100;
        ix.amount_per_period = 10;
        let mut c = contract_with(ix);
        c.amount_withdrawn = 1_000;
        assert_eq!(c.available_to_claim(2_000, 0.0), 0);
    }

    #[test]
    fn available_to_claim_zero_when_zero_deposited() {
        let mut ix = default_params();
        ix.start_time = 1_000;
        ix.net_amount_deposited = 0;
        let c = contract_with(ix);
        assert_eq!(c.available_to_claim(2_000, 0.0), 0);
    }

    #[test]
    fn available_to_claim_returns_remainder_after_end_time_when_not_paused() {
        let mut ix = default_params();
        ix.start_time = 1_000;
        ix.net_amount_deposited = 1_000;
        ix.period = 100;
        ix.amount_per_period = 10;
        let mut c = contract_with(ix);
        c.end_time = 2_000;
        c.amount_withdrawn = 250;
        // at/after end_time with no open pause: net_amount_deposited - amount_withdrawn
        assert_eq!(c.available_to_claim(2_000, 0.0), 750);
        assert_eq!(c.available_to_claim(5_000, 0.0), 750);
    }

    #[test]
    fn available_to_claim_subtracts_amount_withdrawn_for_partial_vest() {
        // `fee_percentage` multiplies the vested/cliff amount via
        // calculate_fee_from_amount, so passing 100.0 makes that multiplication
        // the identity. This isolates the "vested + cliff - amount_withdrawn"
        // logic under test without coupling to a specific fee rate.
        let mut ix = default_params();
        ix.start_time = 1_000;
        ix.net_amount_deposited = 10_000;
        ix.period = 100;
        ix.amount_per_period = 10; // 10 tokens per 100s
        let mut c = contract_with(ix);
        c.end_time = 10_000; // still streaming at now=1500, avoids the end_time early-return
        c.amount_withdrawn = 30;
        // at t=1500: vested_available = (1500-1000)/100 * 10 = 50; cliff=0; claim = 50 - 30 = 20
        assert_eq!(c.available_to_claim(1_500, 100.0), 20);
    }

    #[test]
    fn available_to_claim_includes_cliff_amount() {
        // fee_percentage=100.0 is the identity multiplier (see partial-vest test),
        // so the assertion reflects raw vested + cliff - withdrawn.
        let mut ix = default_params();
        ix.start_time = 1_000;
        ix.cliff = 1_200;
        ix.cliff_amount = 200;
        ix.net_amount_deposited = 10_000;
        ix.period = 100;
        ix.amount_per_period = 10;
        let mut c = contract_with(ix);
        c.end_time = 10_000; // avoid the end_time early-return at now=1200/1300
        // exactly at cliff: vested = 0, cliff = 200, claim = 200
        assert_eq!(c.available_to_claim(1_200, 100.0), 200);
        // 100s after cliff: vested = 1*10 = 10, cliff = 200, claim = 210
        assert_eq!(c.available_to_claim(1_300, 100.0), 210);
    }

    // ---- calculate_fee_from_amount ----

    #[test]
    fn calculate_fee_zero_for_zero_or_negative_percent() {
        assert_eq!(calculate_fee_from_amount(1_000_000, 0.0), 0);
        assert_eq!(calculate_fee_from_amount(1_000_000, -1.5), 0);
    }

    #[test]
    fn calculate_fee_applies_percentage() {
        // 1% of 1_000_000 = 10_000
        assert_eq!(calculate_fee_from_amount(1_000_000, 1.0), 10_000);
        // 0.5% of 1_000_000 = 5_000
        assert_eq!(calculate_fee_from_amount(1_000_000, 0.5), 5_000);
        // 100% of 1_000_000 = 1_000_000
        assert_eq!(calculate_fee_from_amount(1_000_000, 100.0), 1_000_000);
    }

    #[test]
    fn calculate_fee_rounds_down() {
        // 1% of 7 = 0.07, but the function uses integer math with precision 1e6.
        // 1% -> factor = 10_000; amount * 10_000 / 1_000_000 = 7 * 10_000 / 1_000_000 = 0
        assert_eq!(calculate_fee_from_amount(7, 1.0), 0);
        // 50% of 9 = 4.5 -> rounds down to 4
        // factor = 500_000; 9 * 500_000 / 1_000_000 = 4
        assert_eq!(calculate_fee_from_amount(9, 50.0), 4);
    }
}
