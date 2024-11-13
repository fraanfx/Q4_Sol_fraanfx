use anchor_lang::prelude::*;

#[account]

//#[derive] dont forget 8 bytes for 
pub struct Marketplace {
    pub admin; Pubkey,
    pub fee: u16,
    pub treasury_bump: u8,
    pub rewards_bump: u8,
    pub bump: u8,
   #[max_len(32)] 
    pub name: String, // String limited to 32
}

// impl Space for Marketplace { 
//     const INIT_SPACE: usize =  8 + 32 + 2 + 1  + 1 + 1 + (4 + 32) //4 borsch to deserialize + 32 String
// }