use anchor_lang::prelude::*;
use crate::state::multisig::Multisig;
use crate::state::transaction::Transaction;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct RemoveApproval<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut)]
    pub remover: Signer<'info>,
    #[account(mut,
        constraint = transaction.multisig == multisig.key(),
    )]
    pub transaction: Account<'info, Transaction>,
    pub system_program: Program<'info, System>,
}

// Handles the removal of an approval
pub fn remove_approval_handler(ctx: Context<RemoveApproval>) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let transaction = &mut ctx.accounts.transaction;
    let remover = ctx.accounts.remover.key();

    require!(multisig.owners.contains(&remover), ErrorCode::Unauthorized);
    require!(!transaction.executed, ErrorCode::TransactionExecuted);
    require!(transaction.signers.contains(&remover), ErrorCode::NotSigned);

    // Remove the remover from the signers list
    transaction.signers.retain(|&x| x != remover);

    emit!(ApprovalRemoved {
        multisig: multisig.key(),
        signer_removed: remover,
        signers: transaction.signers.len() as u8,
        nonce: transaction.nonce,
    });

    Ok(())
}

#[event]
pub struct ApprovalRemoved {
    pub multisig: Pubkey,
    pub signer_removed: Pubkey,
    pub signers: u8,
    pub nonce: u16,
}