use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum DealState {
    Initialized,
    Funded,
    Completed,
    InDispute,
    Resolved,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct EscrowState {
    pub client_pubkey: Pubkey,
    pub contractor_pubkey: Pubkey,
    pub arbitrator_pubkey: Pubkey,
    pub amount: u64,
    pub deal_state: DealState,
}

impl EscrowState {
    // 32 (client) + 32 (contractor) + 32 (arbitrator) + 8 (amount) + 1 (state)
    pub const LEN: usize = 32 + 32 + 32 + 8 + 1;
}