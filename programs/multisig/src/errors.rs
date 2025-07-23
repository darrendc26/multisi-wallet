use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid number of owners")]
    InvalidNumberOfOwners,
    #[msg("Invalid threshold")]
    InvalidThreshold,
    #[msg("Duplicate owners")]
    DuplicateOwners,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Empty instructions")]
    EmptyInstructions,
    #[msg("Invalid number of instructions")]
    InvalidNumberOfInstructions,
    #[msg("Transaction executed")]
    TransactionExecuted,
    #[msg("Already signed")]
    AlreadySigned,
}