use anchor_lang::prelude::*;
use crate::state::multisig::Multisig;
use crate::state::transaction::{Transaction, InstructionData};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct ProposeTxn<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    #[account(
        init,
        payer = proposer,
        space = 8 + Transaction::INIT_SPACE,
        seeds = [b"transaction".as_ref(), 
        multisig.key().as_ref(), 
        multisig.nonce.to_le_bytes().as_ref()],
        bump
    )]
    pub transaction: Account<'info, Transaction>,
    pub system_program: Program<'info, System>,
}

// Handles the creation of a transaction
pub fn propose_txn_handler(ctx: Context<ProposeTxn>, instruction: Vec<InstructionData>) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let transaction = &mut ctx.accounts.transaction;
    let proposer = ctx.accounts.proposer.key();
    require!(multisig.owners.contains(&proposer), ErrorCode::Unauthorized);
    require!(instruction.len() <= 5, ErrorCode::InvalidNumberOfInstructions);
    require!(!instruction.is_empty(), ErrorCode::EmptyInstructions);

    // Create the transaction account
    transaction.multisig = multisig.key();
    transaction.proposer = proposer;
    transaction.instructions = instruction;

    // Proposer auto approves the transaction
    transaction.signers = vec![proposer];
    transaction.executed = false;
    transaction.bump = ctx.bumps.transaction;
    
    multisig.nonce += 1;
    transaction.nonce = multisig.nonce;

    Ok(())
}