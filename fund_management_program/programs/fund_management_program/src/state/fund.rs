use anchor_lang::prelude::*;
use fixed::types::I80F48;
use crate::errors::FundManagementError;
use crate::instructions::{FundAdministratorAccount, PortfolioManagerAccount};

#[account]
pub struct Fund {
    pub fund_administrator: Pubkey,
    pub portfolio_manager: Pubkey,
    pub fund_vault: Pubkey,
    pub fund_vault_bump: u8,

    pub state: FundState,
    pub last_valuation_ts: u64,
    pub last_valuation: I80F48,

    pub pending_redemptions_counter: u64
}

impl Fund {
    pub fn open(&mut self, fund_administrator: Pubkey, portfolio_manager: Pubkey, fund_vault: Pubkey, bump: u8) -> Result<()> {
        require!(self.state == FundState::Uninitialized, FundManagementError::FundIsAlreadyInitialized);

        self.fund_administrator = fund_administrator;
        self.portfolio_manager = portfolio_manager;
        self.fund_vault = fund_vault;
        self.fund_vault_bump = bump;
        self.state = FundState::Open;
        self.last_valuation = I80F48::from(0);
        self.last_valuation_ts = Clock::get()?.unix_timestamp.try_into().unwrap();
        self.pending_redemptions_counter = 0;

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum FundState {
    Uninitialized,
    Open,
    Close,
}