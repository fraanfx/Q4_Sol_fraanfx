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
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>, //tokens that recives the taker that depositon on vault the maker (a)
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>, //tokens that deposits the taker to transfer to maker (b)
    #[account(

        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>, //tokens deposited from the taker to transfer to maker (b)
    #[account(
        mut,
        close = maker,
        has_one = maker, // has_one checking accounts
        has_one =  mint_a,
        has_one =  mint_b,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump =  escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,


    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

// Deposit tokens from taker to maker

impl<'info> Take<'info> {      
    pub fn deposit(&mut self) -> Result<()> {     //taker deposits the tokens
        let cpi_program = self.token_program.to_account_info();     

        let cpi_accounts = TransferChecked {       // Token B --> (Second payer --> first payer)
            from: self.taker_ata_b.to_account_info() , 
            to: self.maker_ata_b.to_account_info(),    
            authority: self.taker.to_account_info(),
            mint: self.mint_b.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, self.escrow.receive, self.mint_b.decimals);
    }

    
        // Transfer tokens from vault to taker


        pub fn withdraw_and_close_vault(&mut self) -> Result<()> { // At the time that taker transfered the tokens to the maker, the deposit on vault from maker  goes to taker

            let signer_seeds: [&[&[u8]]; 1]  = [&[          
                b"escrow",
                self.maker.key().as_ref(),
                &self.escrow.seed.to_le_bytes()[..],
                &[self.escrow.bump]
            ]];

            let cpi_program =  self.token_program.to_account_info();
    
            let cpi_accounts =  TransferChecked {                   // Token A --> (Vault --> Second payer)
                from: self.vault.to_account_info(),                 
                to: self.taker_ata_a.to_account_info(),
                authority: self.escrow.to_account_info(),
                mint: self.mint_a.to_account_info(),
            }
    
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds); 
            transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;
     
            Ok(())
        }

    // Close vault account

    transfer_checked(cpi_context, self.vault.amount, self.mint_b_decimals)?;

        let cpi_accounts = CloseAccount{    // close account
            account: self.vault.to_account_info(),
            destination: self.taker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds); //new_with_signer PDA is closing the account

        close_account(cpi_context)?; 

        Ok(());

    

    
}
