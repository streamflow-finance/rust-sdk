use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

declare_id!("strmRqUCoQUgGUan5YhzUZa6KqdzwX5L6FpUxfmKg5m");

/// Streamflow sdk module defining anchor account structs expected from the streamflow protocol
/// as well as anchor cpi module used for invoking streamflow protocol
///
/// ## Usage
///
/// Declaring a dependency in program's Cargo.toml
///
/// ```ignore
/// streamflow_sdk = {version = "0.6.0", features = ["cpi"]}
/// ```
///
/// Example anchor program invoking streamflow create instruction
///
///
/// ```ignore
/// use anchor_lang::prelude::*;
/// use anchor_spl::{
///     associated_token::AssociatedToken,
///     token::{Mint, Token, TokenAccount},
/// };
///
/// use streamflow_sdk::cpi::accounts::{
///     Create as CpiCreate,
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
///     //importing parameter struct for stream/vesting contract creation
///     use streamflow_sdk::CreateParams;
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
///     ) -> ProgramResult {
///         let ix = CreateParams {
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
///         };
///
///         // initializing accounts struct for cross-program invoke
///         let accs = CpiCreate {
///             sender: ctx.accounts.sender.to_account_info(),
///             sender_tokens: ctx.accounts.sender_tokens.to_account_info(),
///             recipient: ctx.accounts.recipient.to_account_info(),
///             recipient_tokens: ctx.accounts.recipient_tokens.to_account_info(),
///             metadata: ctx.accounts.metadata.to_account_info(),
///             escrow_tokens: ctx.accounts.escrow_tokens.to_account_info(),
///             streamflow_treasury: ctx.accounts.streamflow_treasury.to_account_info(),
///             streamflow_treasury_tokens:
///             ctx.accounts.streamflow_treasury_tokens.to_account_info(),
///             withdrawor: ctx.accounts.withdrawor.to_account_info(),
///             partner: ctx.accounts.partner.to_account_info(),
///             partner_tokens: ctx.accounts.partner_tokens.to_account_info(),
///             mint: ctx.accounts.mint.to_account_info(),
///             fee_oracle: ctx.accounts.fee_oracle.to_account_info(),
///             rent: ctx.accounts.rent.to_account_info(),
///             timelock_program: ctx.accounts.timelock_program.to_account_info(),
///             token_program: ctx.accounts.token_program.to_account_info(),
///             associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
///             system_program: ctx.accounts.system_program.to_account_info(),
///         };
///
///         // initializing anchor CpiContext, can be used in native solana programs as well
///         // additional reference:
///         // https:///project-serum.github.io/anchor/tutorials/tutorial-3.html
///         let cpi_ctx = CpiContext::new(ctx.accounts.timelock_program.to_account_info(), accs);
///
///         // calling cpi method which calls solana_program invoke with
///         // serialized instruction data fit for streamflow program
///         streamflow_sdk::cpi::create(
///             cpi_ctx,
///             ix.start_time,
///             ix.net_amount_deposited,
///             ix.period,
///             ix.amount_per_period,
///             ix.cliff,
///             ix.cliff_amount,
///             ix.cancelable_by_sender,
///             ix.cancelable_by_recipient,
///             ix.automatic_withdrawal,
///             ix.transferable_by_sender,
///             ix.transferable_by_recipient,
///             ix.can_topup,
///             ix.stream_name,
///             ix.withdraw_frequency,
///         )
///     }
/// ```


#[program]
pub mod streamflow_sdk {
    use super::*;

    /// Anchor rpc handler used for CPI code generation
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

    /// Anchor rpc handler used for CPI code generation
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

    /// Anchor rpc handler used for CPI code generation
    #[allow(unused_variables)]
    pub fn update(
        ctx: Context<Update>,
        enable_automatic_withdrawal: Option<bool>,
        withdraw_frequency: Option<u64>,
        amount_per_period: Option<u64>
    ) -> Result<()> {
        Ok(())
    }

    /// Anchor rpc handler used for CPI code generation
    #[allow(unused_variables)]
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        Ok(())
    }

    /// Anchor rpc handler used for CPI code generation
    #[allow(unused_variables)]
    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        Ok(())
    }

    /// Anchor rpc handler used for CPI code generation
    #[allow(unused_variables)]
    pub fn pause(ctx: Context<Pause>) -> Result<()> {
        Ok(())
    }

    /// Anchor rpc handler used for CPI code generation
    #[allow(unused_variables)]
    pub fn unpause(ctx: Context<UnPause>) -> Result<()> {
        Ok(())
    }

    /// Anchor rpc handler used for CPI code generation
    #[allow(unused_variables)]
    pub fn transfer_recipient(ctx: Context<Transfer>) -> Result<()> {
        Ok(())
    }

    /// Anchor rpc handler used for CPI code generation
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
    /// Expects empty (non-initialized) account.
    #[account(mut)]
    pub escrow_tokens: AccountInfo<'info>,
    /// Associated token account address of `recipient`.
    #[account(mut)]
    pub recipient_tokens: AccountInfo<'info>,
    /// Streamflow treasury account.
    #[account(mut)]
    pub streamflow_treasury: AccountInfo<'info>,
    /// Associated token account address of `streamflow_treasury`.
    #[account(mut)]
    pub streamflow_treasury_tokens: AccountInfo<'info>,
    /// Delegate account for automatically withdrawing contracts.
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
    pub fee_oracle: AccountInfo<'info>,
    /// The Rent Sysvar account.
    pub rent: Sysvar<'info, Rent>,
    /// Streamflow protocol (alias timelock) program account.
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
    /// Associated token account address of `sender`.
    #[account(mut)]
    pub sender_tokens: AccountInfo<'info>,
    /// The account holding the contract parameters.
    /// Expects account initialized with 1104 bytes.
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    /// The escrow account holding the funds.
    /// Expects empty (non-initialized) account.
    #[account(mut)]
    pub escrow_tokens: AccountInfo<'info>,
    /// Delegate account for automatically withdrawing contracts.
    #[account(mut)]
    pub withdrawor: AccountInfo<'info>,
    /// The SPL token mint account.
    pub mint: Account<'info, Mint>,
    /// Internal program that handles fees for specified partners. If no partner fees are expected
    /// on behalf of the program integrating with streamflow, `streamflow_treasury` can be passed
    /// in here.
    pub fee_oracle: AccountInfo<'info>,
    /// The Rent Sysvar account.
    pub rent: Sysvar<'info, Rent>,
    /// Streamflow protocol (alias timelock) program account.
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
    /// Expects initialized account.
    #[account(mut)]
    pub escrow_tokens: Account<'info, TokenAccount>,
    /// Streamflow treasury account.
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
    /// Expects initialized account.
    #[account(mut)]
    pub escrow_tokens: Account<'info, TokenAccount>,
    /// Streamflow treasury account.
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
    /// Expects initialized account.
    #[account(mut)]
    pub escrow_tokens: Account<'info, TokenAccount>,
    /// Streamflow treasury account.
    #[account(mut)]
    pub streamflow_treasury: AccountInfo<'info>,
    /// Associated token account address of `streamflow_treasury`.
    #[account(mut)]
    pub streamflow_treasury_tokens: AccountInfo<'info>,
    /// Delegate account for automatically withdrawing contracts.
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

/// Instruction data expected in create instruction
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
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
    /// Whether or not a stream can be canceled by a sender
    pub cancelable_by_sender: bool,
    /// Whether or not a stream can be canceled by a recipient
    pub cancelable_by_recipient: bool,
    /// Whether or not a 3rd party can initiate withdraw in the name of recipient
    pub automatic_withdrawal: bool,
    /// Whether or not the sender can transfer the stream
    pub transferable_by_sender: bool,
    /// Whether or not the recipient can transfer the stream
    pub transferable_by_recipient: bool,
    /// Whether topup is enabled
    pub can_topup: bool,
    /// The name of this stream
    pub stream_name: [u8; 64],
    /// Withdraw frequency
    pub withdraw_frequency: u64,
}


/// Instruction data expected in the create_unchecked instruction
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
#[repr(C)]
pub struct CreateParamsUnchecked {
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
    /// Whether or not a stream can be canceled by a sender
    pub cancelable_by_sender: bool,
    /// Whether or not a stream can be canceled by a recipient
    pub cancelable_by_recipient: bool,
    /// Whether or not a 3rd party can initiate withdraw in the name of recipient
    pub automatic_withdrawal: bool,
    /// Whether or not the sender can transfer the stream
    pub transferable_by_sender: bool,
    /// Whether or not the recipient can transfer the stream
    pub transferable_by_recipient: bool,
    /// Whether topup is enabled
    pub can_topup: bool,
    /// The name of this stream
    pub stream_name: [u8; 64],
    /// Withdraw frequency
    pub withdraw_frequency: u64,
    /// Pubkey of the contract recipient
    pub recipient: Pubkey,
    /// Pubkey of the fee partner
    pub partner: Pubkey,
}


/// Instruction data expected in update instruction
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
#[repr(C)]
pub struct UpdateParams {
    /// Optionally enable automatic withdrawal
    pub enable_automatic_withdrawal: Option<bool>,
    /// If automatic withdrawal is to be enabled, optionally change withdraw frequency
    pub withdraw_frequency: Option<u64>,
    /// Optionally update amount unlocked per period
    pub amount_per_period: Option<u64>,
}
