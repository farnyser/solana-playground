use anchor_lang::prelude::*;
use fixed::types::I80F48;
use crate::errors::FundManagementError;
use crate::instructions::{FundAdministratorAccount, PortfolioManagerAccount};

#[account]
pub struct Fund {
    pub fund_administrator: Pubkey,
    pub portfolio_manager: Pubkey,

    pub state: FundState,
    pub last_valuation_ts: u64,
    pub last_valuation: I80F48,

    // TODO List of pending redemptions
}

impl Fund {
    pub fn open(&mut self, fund_administrator: Pubkey, portfolio_manager: Pubkey) -> Result<()> {
        require!(self.state == FundState::Uninitialized, FundManagementError::FundIsAlreadyInitialized);

        self.fund_administrator = fund_administrator;
        self.portfolio_manager = portfolio_manager;
        self.state = FundState::Open;
        self.last_valuation = I80F48::from(0);
        self.last_valuation_ts = Clock::get()?.unix_timestamp.try_into().unwrap();

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum FundState {
    Uninitialized,
    Open,
    Close,
}