use anchor_lang::prelude::*;

mod state;
mod instructions;

use instructions::*;

declare_id!("GT5BoQEjQHYUeQ9eS51RffpdjiSwFNCfCMjrdAgvmWbB");


#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit);
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault();
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit();
        ctx.accounts.withdraw_and_close_vault();
    }
}

#[derive(Accounts)]
pub struct Initialize {}
