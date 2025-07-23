use anchor_lang::prelude::*;

// Max number of owners = 5 

#[account]
#[derive(InitSpace)]
pub struct Multisig {
    #[max_len(32 * 5)]
    pub owners: Vec<Pubkey>,
    pub threshold: u64,
    pub bump: u8,
}