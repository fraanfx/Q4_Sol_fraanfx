// Dependicies
use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked}};

use crate::state::Escrow;

// Create context

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct taker<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = taker,
        seeds = [b"escrow", taker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = 8 + Escrow::INIT_SPACE,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

// Deposit tokens from taker to maker

impl<'info> Take<'info> {
    pub fn deposit(&mut self, deposit: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.taker_ata_a.to_account_info() ,
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
            mint: self.mint_b.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, deposit, self.mint_b.decimals)?;

        Ok(())
    }

    
        // Transfer tokens from vault to taker


        pub fn withdraw(&mut self, deposit: u64) -> Result<()> {
            let cpi_program =  self.token_program.to_account_info();
    
            let cpi_accounts =  TransferChecked {
                from: self.vault.to_account_info(),
                to: self.taker_ata_a.to_account_info(),
                authority: self.maker.to_account_info(),
                mint: self.mint_a.to_account_info(),
            }
    
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts); 
            transfer_checked(cpi_ctx, deposit, self.mint_a.decimals)?;
    
            Ok(())
        }

    // Close vault account

    transfer_checked(cpi_context, self.vault.amount, self.mint_b_decimals)?;

        let cpi_accounts = CloseAccount{
            account: self.vault.to_account_info(),
            destination: self.taker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        close_account(cpi_context)?; 

        Ok(());

    

    
}
