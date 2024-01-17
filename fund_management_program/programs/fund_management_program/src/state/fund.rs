use anchor_lang::prelude::*;
use fixed::types::I80F48;

#[account]
pub struct Fund {
    pub fund_administrator: Pubkey,
    pub portfolio_manager: Pubkey,

    pub state: FundState,
    pub last_valuation_ts: u64,
    pub last_valuation: I80F48,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum FundState {
    Open,
    Close,
}