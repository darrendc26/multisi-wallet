use anchor_lang::prelude::*;
use crate::state::multisig::Multisig;
use crate::state::transaction::Transaction;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct ApproveTxn<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut)]
    pub approver: Signer<'info>,
    #[account(mut,
        constraint = transaction.multisig == multisig.key(),
    )]
    pub transaction: Account<'info, Transaction>,
    pub system_program: Program<'info, System>,
}

// Handles the approval of a transaction
pub fn approve_txn_handler(ctx: Context<ApproveTxn>) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let transaction = &mut ctx.accounts.transaction;
    let approver = ctx.accounts.approver.key();

    require!(multisig.owners.contains(&approver), ErrorCode::Unauthorized);
    require!(!transaction.executed, ErrorCode::TransactionExecuted);
    require!(!transaction.signers.contains(&approver), ErrorCode::AlreadySigned);

    // Add the approver to the signers list
    transaction.signers.push(approver);

    emit!(TransactionApproved {
        multisig: multisig.key(),
        signers: transaction.signers.len() as u8,
        nonce: transaction.nonce,
    });

    Ok(())
}

#[event]
pub struct TransactionApproved {
    pub multisig: Pubkey,
    pub signers: u8,
    pub nonce: u16,
}