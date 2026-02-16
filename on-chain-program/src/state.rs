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
    pub instant_payout_amount: u64, // Сумма для мгновенной выплаты (аванс)
    pub deal_state: DealState,
}

impl EscrowState {
    // 32+32+32 (ключи) + 8 (сумма) + 8 (аванс) + 1 (статус)
    pub const LEN: usize = 32 + 32 + 32 + 8 + 8 + 1;
}
