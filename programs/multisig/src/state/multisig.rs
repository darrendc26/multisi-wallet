use anchor_lang::prelude::*;

// Max number of owners = 5 

#[account]
#[derive(InitSpace)]
pub struct Multisig {
    #[max_len(5)]
    pub owners: Vec<Pubkey>,
    pub creator: Pubkey,
    pub threshold: u8,
    pub nonce: u16,
    pub bump: u8,
}