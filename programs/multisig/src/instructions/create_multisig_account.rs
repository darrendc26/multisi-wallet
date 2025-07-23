use anchor_lang::prelude::*;
use crate::state::multisig::Multisig;
use crate::errors::ErrorCode;
#[derive(Accounts)]
pub struct CreateMultisig<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + Multisig::INIT_SPACE, 
        seeds = [b"multisig".as_ref(), owner.key().as_ref()], 
        bump
    )]
    pub multisig: Account<'info, Multisig>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_multisig_handler(ctx: Context<CreateMultisig>, owners: Vec<Pubkey>, threshold: u64) -> Result<()> {
    require!(owners.len() <= 5, ErrorCode::InvalidNumberOfOwners);
    require!(threshold <= owners.len() as u64, ErrorCode::InvalidThreshold);
    require!(threshold > 0, ErrorCode::InvalidThreshold);

    let mut unique_owners = owners.clone();
    unique_owners.sort();
    unique_owners.dedup();
    require!(unique_owners.len() == owners.len(), ErrorCode::DuplicateOwners);

    let multisig = &mut ctx.accounts.multisig;
    multisig.owners = owners;
    multisig.threshold = threshold;
    multisig.nonce = 0;
    multisig.bump = ctx.bumps.multisig;

    Ok(())
}
