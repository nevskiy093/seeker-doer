use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};
use borsh::BorshDeserialize;

pub mod instruction;
pub mod state;
pub mod processor;

use instruction::EscrowInstruction;

// 1. Объявление точки входа (остается без изменений)
entrypoint!(process_instruction);

// 2. Реализация точки входа
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Escrow program entrypoint");

    // Десериализуем данные инструкции, чтобы понять, какую команду вызвали
    let instruction = EscrowInstruction::try_from_slice(instruction_data)?;

    // Вызываем главный обработчик, передавая ему все данные
    processor::process(program_id, accounts, instruction)
}