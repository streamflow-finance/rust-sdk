# streamflow-sdk
SDK for Rust on-chain solana programs to interact with streamflow protocol

## Usage

Declare a dependency in your program's Cargo.toml

```
streamflow_sdk = {version = "0.5.1", features = ["cpi"]}
```

In your program's instruction which should invoke streamflow's create instruction:


```
use anchor_lang::prelude::*;
use streamflow_sdk::cpi::accounts::Create
use streamflow_sdk::cpi;

let accs = Create {
    sender: ctx.accounts.sender.to_account_info(),
    sender_tokens: ctx.accounts.sender_tokens.to_account_info(),
    ...
}

let cpi_ctx = CpiContext::new(
    ctx.accounts.timelock_program.to_account_info(), accs
);

streamflow_sdk::cpi::create(
            cpi_ctx,
            start_time,
            net_amount_deposited
            ...
};

```

## Example program using sdk

For a more detailed example, check ./programs/example/lib.rs 

Adresses
---

| parameter           |address|
|---------------------|----|
| program_id          |addr|
| fees_oracle         |addr|
| streamflow_treasury |5SEpbdjFK5FxwTvfsGMXVQTD2v4M2c5tyRTxhdsPkgDw|
| withdrawor          |wdrwhnCv4pzW8beKsbPa4S2UDZrXenjg16KJdKSpb5u|



## Streamflow protocol


Rust program that provides SPL timelock functionalities

Functionalities are:
- `create` a vesting contract.
- `withdraw` from a vesting contract.
- `cancel` a vesting contract.
- `transfer_recipient` of a vesting contract.

High level overview
--
![Overview](../../misc/overview.jpeg)

## Fees

Streamflow protocol transfer 0.25% of contract total amount (net_amount_deposit) to streamflow treasury to cover costs of building and maintaining protocl. 

## Registering as partner on streamflow protocol

Streamflow protocol supports integrating partner fees for other protocols built on top of streamflow. For more information, reach us out on discord. <link>link</link>