use anchor_lang::prelude::*;

#[account]
pub struct Multisig {
    pub owners: Vec<Pubkey>,
    pub threshold: u64,
    pub bump: u8,
}