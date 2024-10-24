use anchor_lang::prelude::*;

mod state;

declare_id!("GT5BoQEjQHYUeQ9eS51RffpdjiSwFNCfCMjrdAgvmWbB");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
