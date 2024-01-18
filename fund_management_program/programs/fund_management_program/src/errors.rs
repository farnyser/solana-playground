use anchor_lang::error_code;

#[error_code]
pub enum FundManagementError {
    #[msg("Fund is already initialized")]
    FundIsAlreadyInitialized,

    #[msg("Fund is not open for new investment")]
    FundIsNotOpen,

    #[msg("Operation not permitted")]
    InvalidPermission,
}