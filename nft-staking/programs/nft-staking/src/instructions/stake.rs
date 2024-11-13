use anchor_lang::prelude::*;

use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, 
            FreezeDelegatedAccountCpiAccounts
        }, 
        MasterEditionAccount, 
        Metadata, 
        MetadataAccount
    }, 
    token::{
        approve, 
        Approve, 
        Mint, 
        Token, 
        TokenAccount
    }
};

use crate::{errors::StakeError, state::{StakeAccount, StakeConfig, UserAccount}};

#[derive(Accounts)]
pub struct Stake<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>, // NFT Unit reference
    pub collection_mint: Account<'info,  // NFT Collection reference
    #[account(
        mut, 
        associated_token::mint = mint,
        associated_token::authority = user,
    )]

    pub mint_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key.as_ref(), // Check the nft  of the  ollection
        constraint = metadata.collection.as_ref().unwrap().verified = true, // Creck the nft collection reference
    )]
    pub metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds =  [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref(), b"edition"],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>, //Only one could be minted, for nft's
    #[account(
        seeds = [b"config"],
        bump =  config_account.bump,
    )]
    pub config_account: Account<'info, StakeConfig>,
    #[account(
        init,
        payer  = user,
        seeds = [b"stake", config_account.key().as_ref(), mint.key().as_ref()],
        bump,
        space = StakeAccount::INIT_SPACE,
    )]
    pub stake_account: Acccount<'info, StakeAccount>,
    #[account(
        mut,
        seeds =  [b"user", user.key().as_ref()],
        bump =  user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: StakeBumps) -> Result<()> {
        require!(self.user_account.amount_staked < self.config.max_stake, StakeError::MaxStakeReached); // check the ammount of nft stake

        //  Create stake account the nft
        self.stake_account.set_inner(StakeAccount {
            owner: self.user.key(),
            mint: self.mint.key(),
            staked_at: Clock::get()?.unix_timestamp,  
            bump: bumps.stake_account,
        });
      
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Approve {   //CPI that enables delegate property to the stacking program
            to: self.mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.user.to_account_info(),
        };


        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        approve(cpi_ctx, 1)?;

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();


        let seeds = &[
            b"stake",
            self.mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump]
        ];     
        let signer_seeds = &[&seeds[..]];

        FreezeDelegatedAccountCpi::new( // Freezing the nft
            metadata_program,
            FreezeDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program,
            },
        ).invoke_signed(signer_seeds)?;

        //increment the ammount stake by tha ammount of tokens freezed, in this case max 1
        self.user_account.amount_staked += 1;

        Ok(())
    }
}