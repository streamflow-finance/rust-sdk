pub mod state;

use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[cfg(feature = "mainnet")]
declare_id!("strmRqUCoQUgGUan5YhzUZa6KqdzwX5L6FpUxfmKg5m");
#[cfg(not(feature = "mainnet"))]
declare_id!("HqDGZjaVRXJ9MGRQEw7qDc2rAr6iH1n1kAQdCZaCMfMZ");

/// Streamflow sdk module defining anchor account structs expected from the streamflow protocol
/// as well as anchor cpi module used for invoking streamflow protocol
///
/// ## Usage
///
/// Declaring a dependency in program's Cargo.toml
///
/// ```toml
/// streamflow_sdk = {version = "0.6", features = ["cpi"]}
/// ```
///
/// To use protocol on mainnet add sdk with `mainnet` feature
///
/// ```toml
/// streamflow_sdk = {version = "0.6", features = ["cpi", "mainnet"]}
/// ```
///
/// Example anchor program invoking streamflow create instruction
///
///
/// ```rust
/// use anchor_lang::prelude::*;
/// use anchor_spl::{
///     associated_token::AssociatedToken,
///     token::{Mint, Token, TokenAccount},
/// };
///
/// use streamflow_sdk;
/// use streamflow_sdk::cpi::accounts::{
///     Create as CpiCreate,
///     CreateUnchecked as CpiCreateUnchecked,
///     Update as CpiUpdate,
///     Withdraw as CpiWithdraw,
///     Topup as CpiTopup,
///     Transfer as CpiTransfer,
///     Cancel as CpiCancel,
/// };
///
/// declare_id!("FGjLaVo5zLGdzCxMo9gu9tXr1kzTToKd8C8K7YS5hNM1");
///
/// #[program]
/// pub mod example_program {
///     use super::*;
///
///     //anchor rpc handlers
///     pub fn create(
///         ctx: Context<Create>,
///         start_time: u64,
///         net_amount_deposited: u64,
///         period: u64,
///         amount_per_period: u64,
///         cliff: u64,
///         cliff_amount: u64,
///         cancelable_by_sender: bool,
///         cancelable_by_recipient: bool,
///         automatic_withdrawal: bool,
///         transferable_by_sender: bool,
///         transferable_by_recipient: bool,
///         can_topup: bool,
///         stream_name: [u8; 64],
///         withdraw_frequency: u64,
///         pausable: Option<bool>,
///         can_update_rate: Option<bool>,
///     ) -> Result<()> {
///         msg!("Got create");
///         // initializing accounts struct for cross-program invoke
///         let accs = CpiCreate {
///             sender: ctx.accounts.sender.to_account_info(),
///             sender_tokens: ctx.accounts.sender_tokens.to_account_info(),
///             recipient: ctx.accounts.recipient.to_account_info(),
///             recipient_tokens: ctx.accounts.recipient_tokens.to_account_info(),
///             metadata: ctx.accounts.metadata.to_account_info(),
///             escrow_tokens: ctx.accounts.escrow_tokens.to_account_info(),
///             streamflow_treasury: ctx.accounts.streamflow_treasury.to_account_info(),
///             streamflow_treasury_tokens: ctx.accounts.streamflow_treasury_tokens.to_account_info(),
///             withdrawor: ctx.accounts.withdrawor.to_account_info(),
///             partner: ctx.accounts.partner.to_account_info(),
///             partner_tokens: ctx.accounts.partner_tokens.to_account_info(),
///             mint: ctx.accounts.mint.to_account_info(),
///             fee_oracle: ctx.accounts.fee_oracle.to_account_info(),
///             rent: ctx.accounts.rent.to_account_info(),
///             timelock_program: ctx.accounts.streamflow_program.to_account_info(),
///             token_program: ctx.accounts.token_program.to_account_info(),
///             associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
///             system_program: ctx.accounts.system_program.to_account_info(),
///         };
///
///         // initializing anchor CpiContext, can be used in native solana programs as well
///         // additional reference: https://project-serum.github.io/anchor/tutorials/tutorial-3.html
///         let cpi_ctx = CpiContext::new(ctx.accounts.streamflow_program.to_account_info(), accs);
///
///         // calling cpi method which calls solana_program invoke with serialized instruction data
///         // fit for streamflow program
///         streamflow_sdk::cpi::create(
///             cpi_ctx,
///             start_time,
///             net_amount_deposited,
///             period,
///             amount_per_period,
///             cliff,
///             cliff_amount,
///             cancelable_by_sender,
///             cancelable_by_recipient,
///             automatic_withdrawal,
///             transferable_by_sender,
///             transferable_by_recipient,
///             can_topup,
///             stream_name,
///             withdraw_frequency,
///             pausable,
///             can_update_rate
///         )
///     }
/// }
///
/// #[derive(Accounts)]
/// pub struct Create<'info> {
///     #[account(mut)]
///     pub sender: Signer<'info>,
///     #[account(
///         associated_token::mint = mint,
///         associated_token::authority = sender,
///     )]
///     pub sender_tokens: Box<Account<'info, TokenAccount>>,
///     #[account(mut)]
///     /// CHECK: Wallet address of the recipient.
///     pub recipient: UncheckedAccount<'info>,
///     #[account(
///         init_if_needed,
///         payer = sender,
///         associated_token::mint = mint,
///         associated_token::authority = recipient,
///     )]
///     pub recipient_tokens: Box<Account<'info, TokenAccount>>,
///     #[account(mut)]
///     pub metadata: Signer<'info>,
///     #[account(
///         mut,
///         seeds = [b"strm", metadata.key().to_bytes().as_ref()],
///         bump,
///         seeds::program = streamflow_program
///     )]
///     /// CHECK: The escrow account holding the funds, expects empty (non-initialized) account.
///     pub escrow_tokens: AccountInfo<'info>,
///     #[account(mut)]
///     /// CHECK: Streamflow treasury account.
///     pub streamflow_treasury: UncheckedAccount<'info>,
///     #[account(
///         init_if_needed,
///         payer = sender,
///         associated_token::mint = mint,
///         associated_token::authority = streamflow_treasury,
///     )]
///     /// CHECK: Associated token account address of `streamflow_treasury`.
///     pub streamflow_treasury_tokens: Box<Account<'info, TokenAccount>>,
///     #[account(mut)]
///     /// CHECK: Delegate account for automatically withdrawing contracts.
///     pub withdrawor: UncheckedAccount<'info>,
///     #[account(mut)]
///     /// CHECK: Partner treasury account.
///     pub partner: UncheckedAccount<'info>,
///     #[account(
///         init_if_needed,
///         payer = sender,
///         associated_token::mint = mint,
///         associated_token::authority = partner,
///     )]
///     pub partner_tokens: Box<Account<'info, TokenAccount>>,
///     pub mint: Box<Account<'info, Mint>>,
///     /// CHECK: Internal program that handles fees for specified partners.
///     pub fee_oracle: UncheckedAccount<'info>,
///     pub rent: Sysvar<'info, Rent>,
///     /// CHECK: Streamflow protocol (alias timelock) program account.
///     pub streamflow_program: UncheckedAccount<'info>,
///     pub token_program: Program<'info, Token>,
///     pub associated_token_program: Program<'info, AssociatedToken>,
///     pub system_program: Program<'info, System>,
/// }
/// ```


#[program]
pub mod streamflow_sdk {
    use super::*;

    /// Create a Stream
    ///
    /// # Arguments
    ///
    /// * `ctx` - Accounts that will be used on Stream creation
    /// * `start_time` - Unix Timestamp for Stream start, can be 0 to use current time
    /// * `net_amount_deposited` - Amount of Tokens to deposit to the Stream
    /// * `period` - Unlock Period in Seconds, tokens will be unlocked every `period` seconds
    /// * `amount_per_period` - Unlock Amount, every `period` we unlock `amount_per_period` tokens
    /// * `cliff` - Unix Timestamp of Cliff (first unlock), can be 0 to use current time or not use at all
    /// * `cliff_amount` - Cliff Amount of tokens, can 0 to not use Cliff at all
    /// * `cancelable_by_sender` - Whether Stream can by cancelled by Sender
    /// * `cancelable_by_recipient` - Whether Stream can be cancelled by Recipient
    /// * `automatic_withdrawal` - Whether automatic withdrawals are enabled
    /// * `transferable_by_sender` - Whether Stream can be transferred by Sender
    /// * `transferable_by_recipient` - Whether Stream can be transferred by Recipient
    /// * `can_topup` - Whether Stream can be topped up (deposit additional tokens)
    /// * `stream_name` - Name of the Stream
    /// * `withdraw_frequency` - Frequency of Automatic Withdrawals, if `automatic_withdrawal` is on, every `withdraw_frequency` seconds **all unlocked** tokens will be sent to the recipient
    /// * `pausable` - Whether Stream can be paused by Sender
    /// * `can_update_rate` - Whether Sender can update `amount_per_period` value of the stream via `update` method
    #[allow(unused_variables)]
    pub fn create(
        ctx: Context<Create>,
        start_time: u64,
        net_amount_deposited: u64,
        period: u64,
        amount_per_period: u64,
        cliff: u64,
        cliff_amount: u64,
        cancelable_by_sender: bool,
        cancelable_by_recipient: bool,
        automatic_withdrawal: bool,
        transferable_by_sender: bool,
        transferable_by_recipient: bool,
        can_topup: bool,
        stream_name: [u8; 64],
        withdraw_frequency: u64,
        pausable: Option<bool>,
        can_update_rate: Option<bool>,
    ) -> Result<()> {
        Ok(())
    }

    /// Create a Stream and skip some optional checks
    ///
    /// This method allows to create a stream and omit some of address checks on creation.
    /// It is not recommended to use this method unless you need create a stream inside a contract and you don't have space for extra accounts.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Accounts that will be used on Stream creation, `metadata` account shuold be initialized!
    /// * `start_time` - Unix Timestamp for Stream start, can be 0 to use current time
    /// * `net_amount_deposited` - Amount of Tokens to deposit to the Stream
    /// * `period` - Unlock Period in Seconds, tokens will be unlocked every `period` seconds
    /// * `amount_per_period` - Unlock Amount, every `period` we unlock `amount_per_period` tokens
    /// * `cliff` - Unix Timestamp of Cliff (first unlock), can be 0 to use current time or not use at all
    /// * `cliff_amount` - Cliff Amount of tokens, can 0 to not use Cliff at all
    /// * `cancelable_by_sender` - Whether Stream can by cancelled by Sender
    /// * `cancelable_by_recipient` - Whether Stream can be cancelled by Recipient
    /// * `automatic_withdrawal` - Whether automatic withdrawals are enabled
    /// * `transferable_by_sender` - Whether Stream can be transferred by Sender
    /// * `transferable_by_recipient` - Whether Stream can be transferred by Recipient
    /// * `can_topup` - Whether Stream can be topped up (deposit additional tokens)
    /// * `stream_name` - Name of the Stream
    /// * `withdraw_frequency` - Frequency of Automatic Withdrawals, if `automatic_withdrawal` is on, every `withdraw_frequency` seconds **all unlocked** tokens will be sent to the recipient
    /// * `pausable` - Whether Stream can be paused by Sender
    /// * `can_update_rate` - Whether Sender can update `amount_per_period` value of the stream via `update` method

    #[allow(unused_variables)]
    pub fn create_unchecked(
        ctx: Context<CreateUnchecked>,
        start_time: u64,
        net_amount_deposited: u64,
        period: u64,
        amount_per_period: u64,
        cliff: u64,
        cliff_amount: u64,
        cancelable_by_sender: bool,
        cancelable_by_recipient: bool,
        automatic_withdrawal: bool,
        transferable_by_sender: bool,
        transferable_by_recipient: bool,
        can_topup: bool,
        stream_name: [u8; 64],
        withdraw_frequency: u64,
        recipient: Pubkey,
        partner: Pubkey,
        pausable: bool,
        can_update_rate: bool,
    ) -> Result<()> { Ok(()) }

    /// Create a Stream and skip some optional checks
    ///
    /// This method allows to create a stream and omit some of address checks on creation.
    /// Also on creation `payer` account will be used to initiliaze accounts and pay withdrawal fees.
    /// It is not recommended to use this method unless you need create a stream inside a contract and you don't have space for extra accounts
    /// and `sender` can't pay for fees (for example, if `sender` is your contract).
    ///
    /// # Arguments
    ///
    /// * `ctx` - Accounts that will be used on Stream creation, `metadata` account shuold be initialized!
    /// * `start_time` - Unix Timestamp for Stream start, can be 0 to use current time
    /// * `net_amount_deposited` - Amount of Tokens to deposit to the Stream
    /// * `period` - Unlock Period in Seconds, tokens will be unlocked every `period` seconds
    /// * `amount_per_period` - Unlock Amount, every `period` we unlock `amount_per_period` tokens
    /// * `cliff` - Unix Timestamp of Cliff (first unlock), can be 0 to use current time or not use at all
    /// * `cliff_amount` - Cliff Amount of tokens, can 0 to not use Cliff at all
    /// * `cancelable_by_sender` - Whether Stream can by cancelled by Sender
    /// * `cancelable_by_recipient` - Whether Stream can be cancelled by Recipient
    /// * `automatic_withdrawal` - Whether automatic withdrawals are enabled
    /// * `transferable_by_sender` - Whether Stream can be transferred by Sender
    /// * `transferable_by_recipient` - Whether Stream can be transferred by Recipient
    /// * `can_topup` - Whether Stream can be topped up (deposit additional tokens)
    /// * `stream_name` - Name of the Stream
    /// * `withdraw_frequency` - Frequency of Automatic Withdrawals, if `automatic_withdrawal` is on, every `withdraw_frequency` seconds **all unlocked** tokens will be sent to the recipient
    /// * `pausable` - Whether Stream can be paused by Sender
    /// * `can_update_rate` - Whether Sender can update `amount_per_period` value of the stream via `update` method
    #[allow(unused_variables)]
    pub fn create_unchecked_with_payer(
        ctx: Context<CreateUncheckedWithPayer>,
        start_time: u64,
        net_amount_deposited: u64,
        period: u64,
        amount_per_period: u64,
        cliff: u64,
        cliff_amount: u64,
        cancelable_by_sender: bool,
        cancelable_by_recipient: bool,
        automatic_withdrawal: bool,
        transferable_by_sender: bool,
        transferable_by_recipient: bool,
        can_topup: bool,
        stream_name: [u8; 64],
        withdraw_frequency: u64,
        recipient: Pubkey,
        partner: Pubkey,
        pausable: bool,
        can_update_rate: bool,
    ) -> Result<()> { Ok(()) }

    /// Update a Stream
    ///
    /// This method allows to enable automatic withdrawals and/or update `amount_per_period` in a Stream
    ///
    /// # Arguments
    /// * `ctx` - Accounts that will be used on Stream update
    /// * `enable_automatic_withdrawal` - Whether to enable automatic withdrawals (can't be disabled)
    /// * `withdraw_frequency` - Set withdrawal frequency, use it only if `enable_automatic_withdrawal` is set to Some(true)
    /// * `amount_per_period` - Whether to update Unlock Amount of the Stream, `can_update_rate` should be enabled
    #[allow(unused_variables)]
    pub fn update(
        ctx: Context<Update>,
        enable_automatic_withdrawal: Option<bool>,
        withdraw_frequency: Option<u64>,
        amount_per_period: Option<u64>
    ) -> Result<()> {
        Ok(())
    }

    /// Withdraw a Stream
    ///
    /// This methods allows to withdraw a stream, requested amount of tokens will be sent to the Recipient
    /// If `enable_automatic_withdrawal` is set to false only Recipient can request a Withdrawal
    ///
    /// # Arguments
    /// * `ctx` - Accounts that will be used on Stream withdrawal
    /// * `amount` - amount to withdraw, should <= unlocked amount. Use `u64::MAX` if you want to withdraw all unlocked amount
    #[allow(unused_variables)]
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        Ok(())
    }

    /// Cancel a Stream
    ///
    /// Cancels a stream, withdraws all unlocked amount to Recipient and returns all leftover tokens back to Sender
    ///
    /// # Arguments
    /// * `ctx` - Accounts that will be used on Stream cancellation
    #[allow(unused_variables)]
    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        Ok(())
    }

    /// Pause a Stream
    ///
    /// This method pauses a Stream, meaning that no unlocks will be processed, only already unlocked amount can be withdrawn
    ///
    /// # Arguments
    /// * `ctx` - Accounts that will be used on Stream pause
    #[allow(unused_variables)]
    pub fn pause(ctx: Context<Pause>) -> Result<()> {
        Ok(())
    }

    /// Unpause a Stream
    ///
    /// This method unpauses a paused Stream
    ///
    /// # Arguments
    /// * `ctx` - Accounts that will be used on Stream unpause
    #[allow(unused_variables)]
    pub fn unpause(ctx: Context<UnPause>) -> Result<()> {
        Ok(())
    }

    /// Transfer a Stream
    ///
    /// This method transfer Stream to another Recipient, already unlocked amount **won't** be sent to the old Recipient
    /// Because of that this method can be effectively used when if you chose wrong Recipient on Stream creation
    ///
    /// # Arguments
    /// * `ctx` - Accounts that will be used on Stream transfer
    #[allow(unused_variables)]
    pub fn transfer_recipient(ctx: Context<Transfer>) -> Result<()> {
        Ok(())
    }

    /// Topup a Stream
    ///
    /// This method allows to Topup a Stream **if it's not closed**
    ///
    /// # Arguments
    /// * `ctx` - Accounts that will be used on Stream unpause
    /// * `amount` - Amount to Topup a Stream with
    #[allow(unused_variables)]
    pub fn topup(ctx: Context<Topup>, amount: u64) -> Result<()> {
        Ok(())
    }
}

/// Accounts expected in create instruction
#[derive(Accounts)]
pub struct Create<'info> {
    /// Wallet of the contract creator.
    #[account(mut)]
    pub sender: Signer<'info>,
    /// Associated token account address of `sender`.
    #[account(mut)]
    pub sender_tokens: AccountInfo<'info>,
    /// Wallet address of the recipient.
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    /// The account holding the contract parameters.
    /// Expects empty (non-initialized) account.
    #[account(mut, signer)]
    pub metadata: AccountInfo<'info>,
    /// The escrow account holding the funds.
    /// Should be a PDA, use `streamflow_sdk::state::find_escrow_account` to derive
    /// Expects empty (non-initialized) account.
    #[account(mut)]
    pub escrow_tokens: AccountInfo<'info>,
    /// Associated token account address of `recipient`.
    #[account(mut)]
    pub recipient_tokens: AccountInfo<'info>,
    /// Streamflow treasury account.
    /// Use constant `streamflow_sdk::state::STRM_TREASURY`
    #[account(mut)]
    pub streamflow_treasury: AccountInfo<'info>,
    /// Associated token account address of `streamflow_treasury`.
    #[account(mut)]
    pub streamflow_treasury_tokens: AccountInfo<'info>,
    /// Delegate account for automatically withdrawing contracts.
    /// Use constant `streamflow_sdk::state::WITHDRAWOR_ADDRESS`
    #[account(mut)]
    pub withdrawor: AccountInfo<'info>,
    /// Partner treasury account. If no partner fees are expected on behalf of the program
    /// integrating with streamflow, `streamflow_treasury` can be passed in here.
    #[account(mut)]
    pub partner: AccountInfo<'info>,
    /// Associated token account address of `partner`. If no partner fees are expected on behalf of the
    /// program integrating with streamflow, `streamflow_treasury_tokens` can be passed in here.
    #[account(mut)]
    pub partner_tokens: AccountInfo<'info>,
    /// The SPL token mint account.
    pub mint: Account<'info, Mint>,
    /// Internal program that handles fees for specified partners. If no partner fees are expected
    /// on behalf of the program integrating with streamflow, `streamflow_treasury` can be passed
    /// in here.
    /// Use constant `streamflow_sdk::state::FEE_ORACLE_ADDRESS`
    pub fee_oracle: AccountInfo<'info>,
    /// The Rent Sysvar account.
    pub rent: Sysvar<'info, Rent>,
    /// Streamflow protocol (alias timelock) program account.
    /// Use `streamflow_sdk:id()`
    pub timelock_program: AccountInfo<'info>,
    /// The SPL program needed in case an associated account
    /// for the new recipient is being created.
    pub token_program: Program<'info, Token>,
    /// The Associated Token program needed in case associated
    /// account for the new recipient is being created.
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// The Solana system program needed for account creation.
    pub system_program: Program<'info, System>,
}

/// Accounts expected in create_unchecked instruction
#[derive(Accounts)]
pub struct CreateUnchecked<'info> {
    /// Wallet of the contract creator.
    #[account(mut)]
    pub sender: Signer<'info>,
    /// Associated token account address of `sender` for `mint`.
    #[account(mut)]
    pub sender_tokens: AccountInfo<'info>,
    /// The account holding the contract parameters.
    /// Expects account initialized with `streamflow_sdk::state::METADATA_LEN` bytes length.
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    /// The escrow account holding the funds.
    /// Should be a PDA, use `streamflow_sdk::state::find_escrow_account` to derive
    /// Expects empty (non-initialized) account.
    #[account(mut)]
    pub escrow_tokens: AccountInfo<'info>,
    /// Delegate account for automatically withdrawing contracts.
    /// Use constant `streamflow_sdk::state::WITHDRAWOR_ADDRESS`
    #[account(mut)]
    pub withdrawor: AccountInfo<'info>,
    /// The SPL token mint account.
    pub mint: Account<'info, Mint>,
    /// Internal program that handles fees for specified partners. If no partner fees are expected
    /// on behalf of the program integrating with streamflow, `streamflow_treasury` can be passed
    /// in here.
    /// Use constant `streamflow_sdk::state::FEE_ORACLE_ADDRESS`
    pub fee_oracle: AccountInfo<'info>,
    /// The Rent Sysvar account.
    pub rent: Sysvar<'info, Rent>,
    /// Streamflow protocol (alias timelock) program account.
    /// Use `streamflow_sdk:id()`
    pub timelock_program: AccountInfo<'info>,
    /// The SPL program account.
    pub token_program: Program<'info, Token>,
    /// The Solana system program needed for account creation.
    pub system_program: Program<'info, System>,
}

/// Accounts expected in create_unchecked instruction
#[derive(Accounts)]
pub struct CreateUncheckedWithPayer<'info> {
    /// Wallet of the payer account to pay for accounts creation
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Wallet of the contract creator.
    #[account(mut)]
    pub sender: Signer<'info>,
    /// Associated token account address of `sender`.
    #[account(mut)]
    pub sender_tokens: AccountInfo<'info>,
    /// The account holding the contract parameters.
    /// Expects account initialized with 1104 bytes.
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    /// The escrow account holding the funds.
    /// Should be a PDA, use `streamflow_sdk::state::find_escrow_account` to derive
    /// Expects empty (non-initialized) account.
    #[account(mut)]
    pub escrow_tokens: AccountInfo<'info>,
    /// Delegate account for automatically withdrawing contracts.
    /// Use constant `streamflow_sdk::state::WITHDRAWOR_ADDRESS`
    #[account(mut)]
    pub withdrawor: AccountInfo<'info>,
    /// The SPL token mint account.
    pub mint: Account<'info, Mint>,
    /// Internal program that handles fees for specified partners. If no partner fees are expected
    /// on behalf of the program integrating with streamflow, `streamflow_treasury` can be passed
    /// in here.
    /// Use constant `streamflow_sdk::state::FEE_ORACLE_ADDRESS`
    pub fee_oracle: AccountInfo<'info>,
    /// The Rent Sysvar account.
    pub rent: Sysvar<'info, Rent>,
    /// Streamflow protocol (alias timelock) program account.
    /// Use `streamflow_sdk:id()`
    pub timelock_program: AccountInfo<'info>,
    /// The SPL program account.
    pub token_program: Program<'info, Token>,
    /// The Solana system program needed for account creation.
    pub system_program: Program<'info, System>,
}

/// Accounts expected in update instruction
#[derive(Accounts)]
pub struct Update<'info> {
    /// Wallet that initiates contract update.
    #[account(mut)]
    pub sender: Signer<'info>,
    /// The account holding the contract parameters.
    /// Expects initialized account.
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    /// Delegate account for automatically withdrawing contracts.
    /// Use constant `streamflow_sdk::state::WITHDRAWOR_ADDRESS`
    #[account(mut)]
    pub withdrawor: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

/// Accounts expected in pause instruction
#[derive(Accounts)]
pub struct Pause<'info> {
    #[account()]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
}

/// Accounts expected in unpause instruction
#[derive(Accounts)]
pub struct UnPause<'info> {
    #[account()]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
}

/// Accounts expected in withdraw instruction
#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// Wallet of the contract withdrawor.
    #[account()]
    pub authority: Signer<'info>,
    #[account(mut)]
    /// Wallet address of the recipient.
    pub recipient: AccountInfo<'info>,
    /// Associated token account address of `recipient`.
    #[account(mut)]
    pub recipient_tokens: Account<'info, TokenAccount>,
    /// The account holding the contract parameters.
    /// Expects initialized account.
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    /// The escrow account holding the funds.
    /// Should be a PDA, use `streamflow_sdk::state::find_escrow_account` to derive
    /// Expects initialized account.
    #[account(mut)]
    pub escrow_tokens: Account<'info, TokenAccount>,
    /// Streamflow treasury account.
    /// Use constant `streamflow_sdk::state::STRM_TREASURY`
    #[account(mut)]
    pub streamflow_treasury: AccountInfo<'info>,
    /// Associated token account address of `streamflow_treasury`.
    #[account(mut)]
    pub streamflow_treasury_tokens: AccountInfo<'info>,
    /// Partner treasury account. If no partner fees are expected on behalf of the program
    /// integrating with streamflow, `streamflow_treasury` can be passed in here.
    /// Must match partner account in contract metadata.
    #[account(mut)]
    pub partner: AccountInfo<'info>,
    /// Associated token account address of `partner`. If no partner fees are expected on behalf of the
    /// program integrating with streamflow, `streamflow_treasury_tokens` can be passed in here.
    /// Must match partner token account in contract metadata.
    #[account(mut)]
    pub partner_tokens: AccountInfo<'info>,
    /// The SPL token mint account.
    pub mint: Account<'info, Mint>,
    /// The SPL program needed in case an associated account
    /// for the new recipient is being created.
    pub token_program: Program<'info, Token>,
}

/// Accounts expected in cancel instruction
#[derive(Accounts)]
pub struct Cancel<'info> {
    /// Wallet that initiates contract cancel.
    #[account()]
    pub authority: Signer<'info>,
    /// Wallet of the contract creator.
    #[account(mut)]
    pub sender: AccountInfo<'info>,
    /// Associated token account address of `sender`.
    #[account(mut)]
    pub sender_tokens: Account<'info, TokenAccount>,
    /// Wallet address of the recipient.
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    /// Associated token account address of `recipient`.
    #[account(mut)]
    pub recipient_tokens: Account<'info, TokenAccount>,
    /// The account holding the contract parameters.
    /// Expects initialized account.
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    /// The escrow account holding the funds.
    /// Should be a PDA, use `streamflow_sdk::state::find_escrow_account` to derive
    /// Expects initialized account.
    #[account(mut)]
    pub escrow_tokens: Account<'info, TokenAccount>,
    /// Streamflow treasury account.
    /// Use constant `streamflow_sdk::state::STRM_TREASURY`
    #[account(mut)]
    pub streamflow_treasury: AccountInfo<'info>,
    /// Associated token account address of `streamflow_treasury`.
    #[account(mut)]
    pub streamflow_treasury_tokens: AccountInfo<'info>,
    /// Partner treasury account. If no partner fees are expected on behalf of the program
    /// integrating with streamflow, `streamflow_treasury` can be passed in here. Must match partner
    /// account in contract metadata.
    #[account(mut)]
    pub partner: AccountInfo<'info>,
    /// Associated token account address of `partner`. If no partner fees are expected on behalf of the
    /// program integrating with streamflow, `streamflow_treasury_tokens` can be passed in here.
    /// Must match partner token account in contract metadata.
    #[account(mut)]
    pub partner_tokens: AccountInfo<'info>,
    /// The SPL token mint account.
    pub mint: Account<'info, Mint>,
    /// The SPL program needed in case an associated account
    /// for the new recipient is being created.
    pub token_program: Program<'info, Token>,
}

/// Accounts expected in transfer instruction
#[derive(Accounts)]
pub struct Transfer<'info> {
    /// Wallet that initiates contract transfer.
    #[account(mut)]
    pub authority: Signer<'info>,
    /// Wallet address of the new contract recipient
    #[account(mut)]
    pub new_recipient: AccountInfo<'info>,
    /// Wallet address of the new contract recipient's token account
    #[account(mut)]
    pub new_recipient_tokens: AccountInfo<'info>,
    /// The account holding the contract parameters.
    /// Expects initialized account.
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    /// The SPL token mint account.
    pub mint: Account<'info, Mint>,
    /// The Rent Sysvar account.
    pub rent: Sysvar<'info, Rent>,
    /// The SPL program needed in case an associated account
    /// for the new recipient is being created.
    pub token_program: Program<'info, Token>,
    /// The Associated Token program needed in case associated
    /// account for the new recipient is being created.
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// The Solana system program needed for account creation.
    pub system_program: Program<'info, System>,
}

/// Accounts expected in topup instruction
#[derive(Accounts)]
pub struct Topup<'info> {
    /// Wallet of the contract creator.
    #[account(mut)]
    pub sender: Signer<'info>,
    /// Associated token account address of `sender`.
    #[account(mut)]
    pub sender_tokens: AccountInfo<'info>,
    /// The account holding the contract parameters.
    /// Expects initialized account.
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    /// The escrow account holding the funds.
    /// Should be a PDA, use `streamflow_sdk::state::find_escrow_account` to derive
    /// Expects initialized account.
    #[account(mut)]
    pub escrow_tokens: Account<'info, TokenAccount>,
    /// Streamflow treasury account.
    /// Use constant `streamflow_sdk::state::STRM_TREASURY`
    #[account(mut)]
    pub streamflow_treasury: AccountInfo<'info>,
    /// Associated token account address of `streamflow_treasury`.
    #[account(mut)]
    pub streamflow_treasury_tokens: AccountInfo<'info>,
    /// Delegate account for automatically withdrawing contracts.
    /// Use constant `streamflow_sdk::state::WITHDRAWOR_ADDRESS`
    #[account(mut)]
    pub withdrawor: AccountInfo<'info>,
    /// Partner treasury account. If no partner fees are expected on behalf of the program
    /// integrating with streamflow, `streamflow_treasury` can be passed in here. Must match partner
    /// account in contract metadata.
    #[account(mut)]
    pub partner: AccountInfo<'info>,
    /// Associated token account address of `partner`. If no partner fees are expected on behalf of the
    /// program integrating with streamflow, `streamflow_treasury_tokens` can be passed in here.
    /// Must match partner token account in contract metadata.
    #[account(mut)]
    pub partner_tokens: AccountInfo<'info>,
    /// The SPL token mint account.
    pub mint: Account<'info, Mint>,
    /// The SPL program needed in case an associated account
    /// for the new recipient is being created.
    pub token_program: Program<'info, Token>,
    /// The Solana system program needed for account creation.
    pub system_program: Program<'info, System>,
}
