use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Transaction {
    pub multisig: Pubkey,
    pub proposer: Pubkey,
    #[max_len(5)]
    pub instructions: Vec<InstructionData>,
    #[max_len(5)]
    pub signers: Vec<Pubkey>,
    pub executed: bool,
    pub nonce: u16,
    pub bump: u8,
}

// Add InitSpace derive to make it work with Vec in InitSpace context
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct InstructionData {
    pub program_id: Pubkey,
    
    // Add max_len for Vec fields when using InitSpace
    #[max_len(15)]
    pub accounts: Vec<SerializedAccountMeta>,
    
    #[max_len(1232)]
    pub data: Vec<u8>,
}

// Also add InitSpace derive to nested structs
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct SerializedAccountMeta {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}