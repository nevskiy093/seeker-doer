use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
use borsh::BorshDeserialize;

// 1. Объявляем наши модули
pub mod instruction;
pub mod processor;
pub mod state;

// 2. Импортируем наш enum с инструкциями
use crate::instruction::EscrowInstruction;

// 3. Объявляем точку входа в программу
entrypoint!(process_instruction);

// 4. Реализуем главную функцию-диспетчер
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Начало обработки инструкции...");

    // Пытаемся "расшифровать" (десериализовать) данные инструкции, которые пришли от клиента
    let instruction = EscrowInstruction::try_from_slice(instruction_data)
        .map_err(|_| {
            msg!("Ошибка: Не удалось десериализовать данные инструкции!");
            solana_program::program_error::ProgramError::InvalidInstructionData
        })?;

    // Передаем управление в наш главный обработчик (процессор) вместе с расшифрованной инструкцией
    msg!("Инструкция успешно распознана, передаю в процессор.");
    processor::process(program_id, accounts, instruction)
}
