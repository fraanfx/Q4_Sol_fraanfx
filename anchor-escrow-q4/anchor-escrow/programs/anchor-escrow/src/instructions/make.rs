use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked}};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)] //allowing have more than 1 escrow, creates an ID
pub struct Make<'info> {
    pub maker: Signer<'info>,
    pub mint_a:  InterfaceAccount<'info, Mint>,
    pub mint_b:  InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a, // token that sends maker
        associated_token::authority =  maker,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account{
        init,
        payer = maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()], // seed to le bytes to create more than one escrow
        bump,
        space = 8 + Escrow::INIT_SPACE,
    }]
    pub escrow: Account<'info, Escrow>,
    #[account{
        init,
        payer =  maker,
        associated_token::mint = mint_a, 
        associated_token::authority = escrow,
    }]
    pub vault:  InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>, // ever we use init we invoke system program to transfer the ownership

}

impl<'info> Make<'info>{
    pub fn init_escrow(&mut self, seed: u64, receive: u64, bumps: &MakeBumps) -> Result<()> { //&MakeBumps creates cannonical bumps interface //initializing escrow state
        self.escrow.set_inner( //With set_innner we could create a escrow interface with all dependencies
            Escrow {                                        // Create scrow account and create ID
                seed,
                maker: self.maker.key(),
                mint_a: self.mint_a.key(),
                mint_b: self.mint_b.key(),
                receive,
                bumps: bump.escrow,
            }
        );
        Ok(())
    }

    pub fn deposit(&mut self, deposit: u64) -> Result<()> { // transfer mint a to the vault
        let cpi_program =  self.token_program.to_account_info();      

        let cpi_accounts =  TransferChecked { //add extra checks to the transfer 
            from: self.maker_ata_a.to_account_info(),     // Token A --> (First payer --> Vault)
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
            mint: self.mint_a.to_account_info(),
        }

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts); 
        transfer_checked(cpi_ctx, deposit, self.mint_a.decimals)?;

        Ok(())
    }
}