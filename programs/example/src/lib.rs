use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use streamflow_sdk;
use streamflow_sdk::cpi::accounts::{
    Create as CpiCreate,
    CreateUnchecked as CpiCreateUnchecked,
    Update as CpiUpdate,
    Withdraw as CpiWithdraw,
    Topup as CpiTopup,
    Transfer as CpiTransfer,
    Cancel as CpiCancel,
};

declare_id!("FGjLaVo5zLGdzCxMo9gu9tXr1kzTToKd8C8K7YS5hNM1");

#[program]
pub mod example_program {
    use super::*;

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
        pausable: Option<bool>,
        can_update_rate: Option<bool>,
    ) -> Result<()> {
        msg!("Got create");
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
            timelock_program: ctx.accounts.streamflow_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        // initializing anchor CpiContext, can be used in native solana programs as well
        // additional reference: https://project-serum.github.io/anchor/tutorials/tutorial-3.html
        let cpi_ctx = CpiContext::new(ctx.accounts.streamflow_program.to_account_info(), accs);

        // calling cpi method which calls solana_program invoke with serialized instruction data
        // fit for streamflow program
        streamflow_sdk::cpi::create(
            cpi_ctx,
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
            pausable,
            can_update_rate
        )
    }

    pub fn update(
        ctx: Context<Update>,
        enable_automatic_withdrawal: Option<bool>,
        withdraw_frequency: Option<u64>,
        amount_per_period: Option<u64>,
        transferable_by_sender: Option<bool>,
        transferable_by_recipient: Option<bool>,
        cancelable_by_sender: Option<bool>,
    ) -> Result<()> {
        let accs = CpiUpdate {
            sender: ctx.accounts.sender.to_account_info(),
            metadata: ctx.accounts.metadata.to_account_info(),
            withdrawor: ctx.accounts.withdrawor.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.streamflow_program.to_account_info(), accs);
        streamflow_sdk::cpi::update(
            cpi_ctx,
            enable_automatic_withdrawal,
            withdraw_frequency,
            amount_per_period,
            transferable_by_sender,
            transferable_by_recipient,
            cancelable_by_sender,
        )
    }

    //anchor rpc handlers
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
    ) -> Result<()> {
        // initializing accounts struct for cross-program invoke
        let accs = CpiCreateUnchecked {
            sender: ctx.accounts.sender.to_account_info(),
            sender_tokens: ctx.accounts.sender_tokens.to_account_info(),
            metadata: ctx.accounts.metadata.to_account_info(),
            escrow_tokens: ctx.accounts.escrow_tokens.to_account_info(),
            withdrawor: ctx.accounts.withdrawor.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            fee_oracle: ctx.accounts.fee_oracle.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
            timelock_program: ctx.accounts.streamflow_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        // initializing anchor CpiContext, can be used in native solana programs as well
        // additional reference: https://project-serum.github.io/anchor/tutorials/tutorial-3.html
        let cpi_ctx = CpiContext::new(ctx.accounts.streamflow_program.to_account_info(), accs);

        // calling cpi method which calls solana_program invoke with serialized instruction data
        // fit for streamflow program
        streamflow_sdk::cpi::create_unchecked(
            cpi_ctx,
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
            recipient,
            partner,
            pausable,
            can_update_rate
        )
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
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
        let cpi_ctx = CpiContext::new(ctx.accounts.streamflow_program.to_account_info(), accs);
        streamflow_sdk::cpi::withdraw(cpi_ctx, amount)
    }

    pub fn transfer_recipient(ctx: Context<Transfer>) -> Result<()> {
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
        let cpi_ctx = CpiContext::new(ctx.accounts.streamflow_program.to_account_info(), accs);
        streamflow_sdk::cpi::transfer_recipient(cpi_ctx)
    }

    pub fn topup(ctx: Context<Topup>, amount: u64) -> Result<()> {
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
        let cpi_ctx = CpiContext::new(ctx.accounts.streamflow_program.to_account_info(), accs);
        streamflow_sdk::cpi::topup(cpi_ctx, amount)
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
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
        let cpi_ctx = CpiContext::new(ctx.accounts.streamflow_program.to_account_info(), accs);
        streamflow_sdk::cpi::cancel(cpi_ctx)
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = sender,
    )]
    pub sender_tokens: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: Wallet address of the recipient.
    pub recipient: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint,
        associated_token::authority = recipient,
    )]
    pub recipient_tokens: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub metadata: Signer<'info>,
    #[account(
        mut,
        seeds = [b"strm", metadata.key().to_bytes().as_ref()],
        bump,
        seeds::program = streamflow_program
    )]
    /// CHECK: The escrow account holding the funds, expects empty (non-initialized) account.
    pub escrow_tokens: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Streamflow treasury account.
    pub streamflow_treasury: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint,
        associated_token::authority = streamflow_treasury,
    )]
    /// CHECK: Associated token account address of `streamflow_treasury`.
    pub streamflow_treasury_tokens: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: Delegate account for automatically withdrawing contracts.
    pub withdrawor: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Partner treasury account.
    pub partner: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint,
        associated_token::authority = partner,
    )]
    pub partner_tokens: Box<Account<'info, TokenAccount>>,
    pub mint: Box<Account<'info, Mint>>,
    /// CHECK: Internal program that handles fees for specified partners.
    pub fee_oracle: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: Streamflow protocol (alias timelock) program account.
    pub streamflow_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateUnchecked<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = sender,
    )]
    pub sender_tokens: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: The account holding the vesting parameters.
    pub metadata: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: The escrow account holding the funds
    pub escrow_tokens: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Delegate account for automatically withdrawing contracts.
    pub withdrawor: UncheckedAccount<'info>,
    pub mint: Box<Account<'info, Mint>>,
    /// CHECK: Internal program that handles fees for specified partners.
    pub fee_oracle: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: Streamflow protocol (alias timelock) program account.
    pub streamflow_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    /// CHECK: The account holding the vesting parameters.
    pub metadata: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Delegate account for automatically withdrawing contracts.
    pub withdrawor: UncheckedAccount<'info>,
    /// CHECK: Streamflow protocol (alias timelock) program account.
    pub streamflow_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account()]
    pub authority: Signer<'info>,
    #[account(mut)]
    /// CHECK: Wallet address of the recipient.
    pub recipient: UncheckedAccount<'info>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = recipient,
    )]
    pub recipient_tokens: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: The account holding the vesting parameters.
    pub metadata: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: The escrow account holding the funds
    pub escrow_tokens: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Streamflow treasury account.
    pub streamflow_treasury: UncheckedAccount<'info>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = streamflow_treasury,
    )]
    /// CHECK: Associated token account address of `streamflow_treasury`.
    pub streamflow_treasury_tokens: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: Partner treasury account.
    pub partner: UncheckedAccount<'info>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = partner,
    )]
    pub partner_tokens: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub mint:Box<Account<'info, Mint>>,
    /// CHECK: Streamflow protocol (alias timelock) program account.
    pub streamflow_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    /// CHECK: Wallet address of the sender
    pub sender: UncheckedAccount<'info>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = sender,
    )]
    pub sender_tokens: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: Wallet address of the recipient.
    pub recipient: UncheckedAccount<'info>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = recipient,
    )]
    pub recipient_tokens: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: The account holding the vesting parameters.
    pub metadata: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: The escrow account holding the funds
    pub escrow_tokens: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Streamflow treasury account.
    pub streamflow_treasury: UncheckedAccount<'info>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = streamflow_treasury,
    )]
    /// CHECK: Associated token account address of `streamflow_treasury`.
    pub streamflow_treasury_tokens: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: Partner treasury account.
    pub partner: UncheckedAccount<'info>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = partner,
    )]
    pub partner_tokens: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub mint: Box<Account<'info, Mint>>,
    /// CHECK: Streamflow protocol (alias timelock) program account.
    pub streamflow_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    /// CHECK: Wallet address of the recipient.
    pub new_recipient: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = new_recipient,
    )]
    pub new_recipient_tokens: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: The account holding the vesting parameters.
    pub metadata: UncheckedAccount<'info>,
    pub mint: Box<Account<'info, Mint>>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: Streamflow protocol (alias timelock) program account.
    pub streamflow_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Topup<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = sender,
    )]
    pub sender_tokens: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: The account holding the vesting parameters.
    pub metadata: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: The escrow account holding the funds
    pub escrow_tokens: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Streamflow treasury account.
    pub streamflow_treasury: UncheckedAccount<'info>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = streamflow_treasury,
    )]
    /// CHECK: Associated token account address of `streamflow_treasury`.
    pub streamflow_treasury_tokens: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: Delegate account for automatically withdrawing contracts.
    pub withdrawor: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Partner treasury account.
    pub partner: UncheckedAccount<'info>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = partner,
    )]
    pub partner_tokens: Box<Account<'info, TokenAccount>>,
    pub mint: Box<Account<'info, Mint>>,
    /// CHECK: Streamflow protocol (alias timelock) program account.
    pub streamflow_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
