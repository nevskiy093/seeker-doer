use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
    program::invoke,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use borsh::{BorshSerialize, BorshDeserialize};
use crate::instruction::{EscrowInstruction, Resolution};
use crate::state::{EscrowState, DealState};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction: EscrowInstruction,
) -> ProgramResult {
    match instruction {
        EscrowInstruction::InitializeEscrow { amount, instant_payout_amount } => {
            msg!("Instruction: InitializeEscrow");
            process_initialize_escrow(program_id, accounts, amount, instant_payout_amount)
        }
        EscrowInstruction::FundEscrow => {
            msg!("Instruction: FundEscrow");
            process_fund_escrow(program_id, accounts)
        }
        EscrowInstruction::RaiseDispute => {
            msg!("Instruction: RaiseDispute");
            process_raise_dispute(program_id, accounts)
        }
        EscrowInstruction::ResolveDispute { resolution } => {
            msg!("Instruction: ResolveDispute");
            process_resolve_dispute(program_id, accounts, resolution)
        }
    }
}

pub fn process_initialize_escrow(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
    instant_payout_amount: u64,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let initializer = next_account_info(account_iter)?;
    let escrow_account = next_account_info(account_iter)?;
    let client_account = next_account_info(account_iter)?;
    let contractor_account = next_account_info(account_iter)?;
    let arbitrator_account = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;
    let rent_sysvar_account = next_account_info(account_iter)?;
    let rent = &Rent::from_account_info(rent_sysvar_account)?;

    if !initializer.is_signer {
        msg!("Error: Initializer must be a signer.");
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    if instant_payout_amount > amount {
        msg!("Error: Instant payout amount cannot be greater than the total amount.");
        return Err(ProgramError::InvalidInstructionData);
    }

    if escrow_account.owner != system_program.key {
        msg!("Error: Escrow account is already in use.");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Создаем аккаунт для хранения состояния сделки прямо в блокчейне
    invoke(
        &system_instruction::create_account(
            initializer.key,
            escrow_account.key,
            rent.minimum_balance(EscrowState::LEN),
            EscrowState::LEN as u64,
            program_id,
        ),
        &[initializer.clone(), escrow_account.clone(), system_program.clone()],
    )?;

    // Записываем всю информацию о сделке в созданный аккаунт
    let mut escrow_state = EscrowState::try_from_slice(&escrow_account.data.borrow_mut())?;

    escrow_state.client_pubkey = *client_account.key;
    escrow_state.contractor_pubkey = *contractor_account.key;
    escrow_state.arbitrator_pubkey = *arbitrator_account.key;
    escrow_state.amount = amount;
    escrow_state.instant_payout_amount = instant_payout_amount;
    escrow_state.deal_state = DealState::Initialized;
    
    escrow_state.serialize(&mut &mut escrow_account.data.borrow_mut()[..])?;
    
    msg!("Escrow initialized for amount: {}", amount);
    Ok(())
}

pub fn process_fund_escrow(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let client_account = next_account_info(account_iter)?;
    let escrow_account = next_account_info(account_iter)?;

    if !client_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if *escrow_account.owner != *program_id {
        return Err(ProgramError::IllegalOwner);
    }
    
    let mut escrow_state = EscrowState::try_from_slice(&escrow_account.data.borrow())?;

    if escrow_state.client_pubkey != *client_account.key {
        msg!("Error: Only the client can fund the escrow.");
        return Err(ProgramError::InvalidAccountData);
    }

    if escrow_state.deal_state != DealState::Initialized {
        msg!("Error: Escrow can only be funded once.");
        return Err(ProgramError::InvalidAccountData);
    }

    // ВАЖНО: Сама пересылка средств происходит на стороне клиента (в коде веб-приложения).
    // Клиент должен переслать (total_amount - instant_payout_amount) на эскроу-счет
    // и instant_payout_amount напрямую на счет исполнителя.
    // Эта инструкция только меняет состояние, подтверждая факт финансирования.

    escrow_state.deal_state = DealState::Funded;
    escrow_state.serialize(&mut &mut escrow_account.data.borrow_mut()[..])?;
    
    msg!("Escrow account has been funded.");
    Ok(())
}

pub fn process_raise_dispute(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    // Эта функция осталась без изменений
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
        return Err(ProgramError::InvalidAccountData);
    }
    if escrow_state.deal_state != DealState::Funded {
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
    // Эта функция осталась без изменений
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
        return Err(ProgramError::InvalidAccountData);
    }
    if escrow_state.deal_state != DealState::InDispute {
        return Err(ProgramError::InvalidAccountData);
    }
    let remaining_amount = escrow_account.lamports();
    match resolution {
        Resolution::RefundToClient => {
            **escrow_account.try_borrow_mut_lamports()? -= remaining_amount;
            **client_account.try_borrow_mut_lamports()? += remaining_amount;
        }
        Resolution::PayToContractor => {
            **escrow_account.try_borrow_mut_lamports()? -= remaining_amount;
            **contractor_account.try_borrow_mut_lamports()? += remaining_amount;
        }
    }
    escrow_state.amount = 0;
    escrow_state.deal_state = DealState::Resolved;
    escrow_state.serialize(&mut &mut escrow_account.data.borrow_mut()[..])?;
    Ok(())
}
