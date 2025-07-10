# streamflow-sdk
SDK for Rust on-chain solana programs to interact with streamflow protocol

## Usage

Declare a dependency in your program's Cargo.toml

```toml
streamflow_sdk = {version = "0.10", features = ["cpi"]}
```

To use protocol on devnet add sdk with `devnet` feature

```toml
streamflow_sdk = {version = "0.10", features = ["cpi", "devnet"]}
```

Example anchor program invoking streamflow create instruction:

```rust
use anchor_lang::{prelude::*};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use streamflow_sdk;
use streamflow_sdk::cpi::accounts::Create;

let accs = Create {
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

let cpi_ctx = CpiContext::new(ctx.accounts.streamflow_program.to_account_info(), accs);

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
```

## Example program using sdk

For a more detailed example, check ./programs/example/lib.rs

Addresses
---

| parameter           |address|
|---------------------|----|
| program_id          |strmRqUCoQUgGUan5YhzUZa6KqdzwX5L6FpUxfmKg5m|
| fees_oracle         |addr|
| streamflow_treasury |5SEpbdjFK5FxwTvfsGMXVQTD2v4M2c5tyRTxhdsPkgDw|
| withdrawor          |wdrwhnCv4pzW8beKsbPa4S2UDZrXenjg16KJdKSpb5u|



## Streamflow protocol


Rust program that provides SPL timelock functionalities

Functionalities are:
- `create` a vesting contract.
- `update` a vesting contract
- `withdraw` from a vesting contract.
- `cancel` a vesting contract.
- `transfer_recipient` of a vesting contract.

**Security audit passed ✅**

Protocol audit available [here](https://github.com/streamflow-finance/rust-sdk/blob/main/protocol_audit.pdf).

Partner oracle audit available here [here](https://github.com/streamflow-finance/rust-sdk/blob/main/partner_oracle_audit.pdf).

High level overview
--
![Overview](./misc/overview.jpeg)

## Fees

The protocol is free.

## Registering as partner on streamflow protocol

Streamflow protocol supports integrating partner fees for other protocols built on top of streamflow. For more information, reach us out on discord. https://discord.gg/NPxw3m4pzt
