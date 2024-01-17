use anchor_lang::error_code;

#[error_code]
pub enum FundManagementError {
    #[msg("Fund is already initialized")]
    FundIsAlreadyInitialized,

    #[msg("Fund is closed")]
    FundIsClosed,

    #[msg("Operation not permitted")]
    InvalidPermission,
}