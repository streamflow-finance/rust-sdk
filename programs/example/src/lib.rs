use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use streamflow_sdk::cpi::accounts::{
    Create as CpiCreate,
    Withdraw as CpiWithdraw,
    Topup as CpiTopup,
    Transfer as CpiTransfer,
    Cancel as CpiCancel,
};

declare_id!("FGjLaVo5zLGdzCxMo9gu9tXr1kzTToKd8C8K7YS5hNM1");

#[program]
pub mod example_program {
    use super::*;

    //importing parameter struct for stream/vesting contract creation
    use streamflow_sdk::CreateParams;

    //anchor rpc handlers
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
        let ix = CreateParams {
            start_time,
            net_amount_deposited,
            period,
            amount_per_period,
            cliff,
            cliff_amount,
            cancelable_by_sender,
            cancelable_by_recipient,
            automatic_withdrawal,
            transferable_by_sender,
            transferable_by_recipient,
            can_topup,
            stream_name,
            withdraw_frequency,
        };

        // initializing accounts struct for cross-program invoke
        let accs = CpiCreate {
            sender: ctx.accounts.sender.to_account_info(),
            sender_tokens: ctx.accounts.sender_tokens.to_account_info(),
            recipient: ctx.accounts.recipient.to_account_info(),
            recipient_tokens: ctx.accounts.recipient_tokens.to_account_info(),
            metadata: ctx.accounts.metadata.to_account_info(),
            escrow_tokens: ctx.accounts.escrow_tokens.to_account_info(),
            streamflow_treasury: ctx.accounts.streamflow_treasury.to_account_info(),
            streamflow_treasury_tokens: ctx.accounts.streamflow_treasury_tokens.to_account_info(),
            withdrawor: ctx.accounts.withdrawor.to_account_info(),
            partner: ctx.accounts.partner.to_account_info(),
            partner_tokens: ctx.accounts.partner_tokens.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            fee_oracle: ctx.accounts.fee_oracle.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
            timelock_program: ctx.accounts.timelock_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        // initializing anchor CpiContext, can be used in native solana programs as well
        // additional reference: https://project-serum.github.io/anchor/tutorials/tutorial-3.html
        let cpi_ctx = CpiContext::new(ctx.accounts.timelock_program.to_account_info(), accs);

        // calling cpi method which calls solana_program invoke with serialized instruction data
        // fit for streamflow program
        streamflow_sdk::cpi::create(
            cpi_ctx,
            ix.start_time,
            ix.net_amount_deposited,
            ix.period,
            ix.amount_per_period,
            ix.cliff,
            ix.cliff_amount,
            ix.cancelable_by_sender,
            ix.cancelable_by_recipient,
            ix.automatic_withdrawal,
            ix.transferable_by_sender,
            ix.transferable_by_recipient,
            ix.can_topup,
            ix.stream_name,
            ix.withdraw_frequency,
        )
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let accs = CpiWithdraw {
            authority: ctx.accounts.authority.to_account_info(),
            recipient: ctx.accounts.recipient.to_account_info(),
            recipient_tokens: ctx.accounts.recipient_tokens.to_account_info(),
            metadata: ctx.accounts.metadata.to_account_info(),
            escrow_tokens: ctx.accounts.escrow_tokens.to_account_info(),
            streamflow_treasury: ctx.accounts.streamflow_treasury.to_account_info(),
            streamflow_treasury_tokens: ctx.accounts.streamflow_treasury_tokens.to_account_info(),
            partner: ctx.accounts.partner.to_account_info(),
            partner_tokens: ctx.accounts.partner_tokens.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.timelock_program.to_account_info(), accs);
        streamflow_sdk::cpi::withdraw(cpi_ctx, amount)
    }

    pub fn cancel(ctx: Context<Cancel>) -> ProgramResult {
        let accs = CpiCancel {
            authority: ctx.accounts.authority.to_account_info(),
            sender: ctx.accounts.sender.to_account_info(),
            sender_tokens: ctx.accounts.sender_tokens.to_account_info(),
            recipient: ctx.accounts.recipient.to_account_info(),
            recipient_tokens: ctx.accounts.recipient_tokens.to_account_info(),
            metadata: ctx.accounts.metadata.to_account_info(),
            escrow_tokens: ctx.accounts.escrow_tokens.to_account_info(),
            streamflow_treasury: ctx.accounts.streamflow_treasury.to_account_info(),
            streamflow_treasury_tokens: ctx.accounts.streamflow_treasury_tokens.to_account_info(),
            partner: ctx.accounts.partner.to_account_info(),
            partner_tokens: ctx.accounts.partner_tokens.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.timelock_program.to_account_info(), accs);
        streamflow_sdk::cpi::cancel(cpi_ctx)
    }
}

pub fn transfer_recipient(ctx: Context<Transfer>) -> ProgramResult {
    let accs = CpiTransfer {
        authority: ctx.accounts.authority.to_account_info(),
        new_recipient: ctx.accounts.new_recipient.to_account_info(),
        new_recipient_tokens: ctx.accounts.new_recipient_tokens.to_account_info(),
        metadata: ctx.accounts.metadata.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.timelock_program.to_account_info(), accs);
    streamflow_sdk::cpi::transfer_recipient(cpi_ctx)
}

pub fn topup(ctx: Context<Topup>, amount: u64) -> ProgramResult {
    let accs = CpiTopup {
        sender: ctx.accounts.sender.to_account_info(),
        sender_tokens: ctx.accounts.sender_tokens.to_account_info(),
        metadata: ctx.accounts.metadata.to_account_info(),
        escrow_tokens: ctx.accounts.escrow_tokens.to_account_info(),
        streamflow_treasury: ctx.accounts.streamflow_treasury.to_account_info(),
        streamflow_treasury_tokens: ctx.accounts.streamflow_treasury_tokens.to_account_info(),
        withdrawor: ctx.accounts.withdrawor.to_account_info(),
        partner: ctx.accounts.partner.to_account_info(),
        partner_tokens: ctx.accounts.partner_tokens.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.timelock_program.to_account_info(), accs);
    streamflow_sdk::cpi::topup(cpi_ctx, amount)
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
    pub timelock_program: AccountInfo<'info>,
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
    pub timelock_program: AccountInfo<'info>,
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
    pub timelock_program: AccountInfo<'info>,
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
    pub timelock_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
