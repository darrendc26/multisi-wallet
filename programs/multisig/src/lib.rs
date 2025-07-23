#![allow(deprecated)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod instructions;

use instructions::*;
use state::transaction::InstructionData;
declare_id!("GLtnCFBPrFgisKinYNw5sW9s3VHM5J8KvtBncQfUWjY6");

#[program]
pub mod multisig {
    use super::*;

    // Create a multisig account
    pub fn create_multisig(ctx: Context<CreateMultisig>, owners: Vec<Pubkey>, threshold: u8) -> Result<()> {
        create_multisig_account::create_multisig_handler(ctx, owners, threshold)
    }

    // Propose a transaction
    pub fn propose_txn(ctx: Context<ProposeTxn>, instruction: Vec<InstructionData>) -> Result<()> {
        propose_txn_handler(ctx, instruction)
    }

    // Approve a transaction
    pub fn approve_txn(ctx: Context<ApproveTxn>) -> Result<()> {
        approve_txn_handler(ctx)
    }
}
