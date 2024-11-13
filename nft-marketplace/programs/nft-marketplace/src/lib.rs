use anchor_lang::prelude::*;

declare_id!("9nG1kUPJtKLPFggKCpGV61tTzGBJobc8NSgCtVRbyQm1");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
