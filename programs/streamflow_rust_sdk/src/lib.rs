use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("96XymQipCZuhyqWinnw4kDJGFuxeYKTSp7PTd9cGAzge");

#[program]
pub mod streamflow_rust_sdk {
    use super::*;

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
    ) -> ProgramResult {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn cancel(ctx: Context<Cancel>) -> ProgramResult {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn transfer_recipient(ctx: Context<Transfer>) -> ProgramResult {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn topup(ctx: Context<Topup>, amount: u64) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub sender_tokens: AccountInfo<'info>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    #[account(mut, signer)]
    pub metadata: AccountInfo<'info>,
    #[account(mut)]
    pub escrow_tokens: AccountInfo<'info>,
    #[account(mut)]
    pub recipient_tokens: AccountInfo<'info>,
    #[account(mut)]
    pub streamflow_treasury: AccountInfo<'info>,
    #[account(mut)]
    pub streamflow_treasury_tokens: AccountInfo<'info>,
    #[account(mut)]
    pub withdrawor: AccountInfo<'info>,
    #[account(mut)]
    pub partner: AccountInfo<'info>,
    #[account(mut)]
    pub partner_tokens: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    pub fee_oracle: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub timelock_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account()]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    #[account(mut)]
    pub recipient_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    #[account(mut)]
    pub escrow_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub streamflow_treasury: AccountInfo<'info>,
    #[account(mut)]
    pub streamflow_treasury_tokens: AccountInfo<'info>,
    #[account(mut)]
    pub partner: AccountInfo<'info>,
    #[account(mut)]
    pub partner_tokens: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account()]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub sender: AccountInfo<'info>,
    #[account(mut)]
    pub sender_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    #[account(mut)]
    pub recipient_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    #[account(mut)]
    pub escrow_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub streamflow_treasury: AccountInfo<'info>,
    #[account(mut)]
    pub streamflow_treasury_tokens: AccountInfo<'info>,
    #[account(mut)]
    pub partner: AccountInfo<'info>,
    #[account(mut)]
    pub partner_tokens: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub new_recipient: AccountInfo<'info>,
    #[account(mut)]
    pub new_recipient_tokens: AccountInfo<'info>,
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Topup<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub sender_tokens: AccountInfo<'info>,
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    #[account(mut)]
    pub escrow_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub streamflow_treasury: AccountInfo<'info>,
    #[account(mut)]
    pub streamflow_treasury_tokens: AccountInfo<'info>,
    #[account(mut)]
    pub withdrawor: AccountInfo<'info>,
    #[account(mut)]
    pub partner: AccountInfo<'info>,
    #[account(mut)]
    pub partner_tokens: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
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
