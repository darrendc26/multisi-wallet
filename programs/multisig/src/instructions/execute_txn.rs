use anchor_lang::prelude::*;
use crate::state::multisig::Multisig;
use crate::state::transaction::Transaction;
use crate::errors::ErrorCode;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::instruction::{Instruction, AccountMeta};

#[derive(Accounts)]
pub struct ExecuteTxn<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut,
        constraint = transaction.multisig == multisig.key(),
        constraint = transaction.executed == false,
    )]
    pub transaction: Account<'info, Transaction>,
    pub executor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Handles the execution of a transaction
pub fn execute_txn_handler(ctx: Context<ExecuteTxn>) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let transaction = &mut ctx.accounts.transaction;

    require!(transaction.signers.len() >= multisig.threshold as usize, ErrorCode::NotEnoughSigners);

    // Execute the transaction
    let seeds = &[
        b"multisig".as_ref(),
        multisig.creator.as_ref(),
        &[multisig.bump],
    ];

    for (index, instruction_data) in transaction.instructions.iter().enumerate() {
        msg!("Instruction {}", index + 1);
        let accounts = instruction_data
                        .accounts.iter().map(|acc| {
                            AccountMeta {
                                pubkey: acc.pubkey,
                                is_signer: acc.is_signer,
                                is_writable: acc.is_writable,
                            }
                        }).collect();
                        
        let ix = Instruction {
            program_id: instruction_data.program_id,
            accounts: accounts,
            data: instruction_data.data.clone(),
        };

        let instruction_accounts = get_instruction_accounts(
                &ix,
                ctx.remaining_accounts,
                multisig.key(),
        )?;

        invoke_signed(
            &ix,
            &instruction_accounts,
            &[&seeds[..]],
        )?;
    }

    transaction.executed = true;

    emit!(TransactionExecuted {
        multisig: multisig.key(),
        nonce: transaction.nonce,
    });

    Ok(())
}

#[event]
pub struct TransactionExecuted {
    pub multisig: Pubkey,
    pub nonce: u16,
}

// Helper function to map instruction accounts to remaining accounts
fn get_instruction_accounts<'info>(
    instruction: &Instruction,
    remaining_accounts: &[AccountInfo<'info>],
    multisig_key: Pubkey,
) -> Result<Vec<AccountInfo<'info>>> {
    let mut accounts = Vec::new();
    
    for account_meta in &instruction.accounts {
        // Find matching account in remaining_accounts by pubkey
        let account_info = if account_meta.pubkey == multisig_key {
            // Special case: if the account is the multisig PDA, we need to use the multisig account
            // This should be handled by passing the multisig account in remaining_accounts
            remaining_accounts.iter()
                .find(|acc| acc.key() == account_meta.pubkey)
                .ok_or(ErrorCode::MissingAccount)?
        } else {
            // For other accounts, find by pubkey
            remaining_accounts.iter()
                .find(|acc| acc.key() == account_meta.pubkey)
                .ok_or(ErrorCode::MissingAccount)?
        };

        accounts.push(account_info.clone());
    }
    
    Ok(accounts)
}