use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
};
use borsh::{BorshSerialize, BorshDeserialize};
use crate::instruction::{EscrowInstruction, Resolution};
use crate::state::{EscrowState, DealState};

// ... (process, process_initialize_escrow, process_fund_escrow, process_release_prepayment, process_confirm_work_done без изменений)

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction: EscrowInstruction,
) -> ProgramResult {
    match instruction {
        // ... (предыдущие обработчики)
        EscrowInstruction::RaiseDispute => {
            msg!("Instruction: RaiseDispute");
            process_raise_dispute(program_id, accounts)
        }
        EscrowInstruction::ResolveDispute { resolution } => {
            msg!("Instruction: ResolveDispute");
            process_resolve_dispute(program_id, accounts, resolution)
        }
        // ... (остальные обработчики)
    }
}

pub fn process_raise_dispute(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let signer_account = next_account_info(account_iter)?;
    let escrow_account = next_account_info(account_iter)?;

    if !signer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if *escrow_account.owner != *program_id {
        return Err(ProgramError::IllegalOwner);
    }

    let mut escrow_state = EscrowState::try_from_slice(&escrow_account.data.borrow())?;

    if *signer_account.key != escrow_state.client_pubkey && *signer_account.key != escrow_state.contractor_pubkey {
        msg!("Error: Signer must be the client or the contractor.");
        return Err(ProgramError::InvalidAccountData);
    }

    if escrow_state.deal_state != DealState::Funded {
        msg!("Error: Dispute can only be raised when the deal is funded.");
        return Err(ProgramError::InvalidAccountData);
    }

    escrow_state.deal_state = DealState::InDispute;
    escrow_state.serialize(&mut &mut escrow_account.data.borrow_mut()[..])?;

    Ok(())
}

pub fn process_resolve_dispute(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    resolution: Resolution,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let arbitrator_account = next_account_info(account_iter)?;
    let escrow_account = next_account_info(account_iter)?;
    let client_account = next_account_info(account_iter)?;
    let contractor_account = next_account_info(account_iter)?;

    if !arbitrator_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if *escrow_account.owner != *program_id {
        return Err(ProgramError::IllegalOwner);
    }

    let mut escrow_state = EscrowState::try_from_slice(&escrow_account.data.borrow())?;

    if escrow_state.arbitrator_pubkey != *arbitrator_account.key {
        msg!("Error: Only the arbitrator can resolve a dispute.");
        return Err(ProgramError::InvalidAccountData);
    }

    if escrow_state.deal_state != DealState::InDispute {
        msg!("Error: Dispute can only be resolved when the deal is in dispute.");
        return Err(ProgramError::InvalidAccountData);
    }

    let remaining_amount = escrow_state.amount;

    match resolution {
        Resolution::RefundToClient => {
            msg!("Dispute resolved: Refunding to client.");
            **escrow_account.try_borrow_mut_lamports()? -= remaining_amount;
            **client_account.try_borrow_mut_lamports()? += remaining_amount;
        }
        Resolution::PayToContractor => {
            msg!("Dispute resolved: Paying to contractor.");
            **escrow_account.try_borrow_mut_lamports()? -= remaining_amount;
            **contractor_account.try_borrow_mut_lamports()? += remaining_amount;
        }
    }

    escrow_state.amount = 0;
    escrow_state.deal_state = DealState::Resolved;
    escrow_state.serialize(&mut &mut escrow_account.data.borrow_mut()[..])?;

    Ok(())
}
