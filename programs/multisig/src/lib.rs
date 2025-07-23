#![allow(deprecated)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod instructions;

use instructions::*;

declare_id!("GLtnCFBPrFgisKinYNw5sW9s3VHM5J8KvtBncQfUWjY6");

#[program]
pub mod multisig {
    use super::*;

    pub fn create_multisig(ctx: Context<CreateMultisig>, owners: Vec<Pubkey>, threshold: u64) -> Result<()> {
        create_multisig_account::create_multisig_handler(ctx, owners, threshold)
    }
}
