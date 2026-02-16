use borsh::{BorshDeserialize, BorshSerialize};

// Варианты решения спора арбитром
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Resolution {
    RefundToClient,
    PayToContractor,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum EscrowInstruction {
    /**
     * Инициирует сделку. Может быть вызвана любой из сторон.
     *
     * amount: Общая сумма сделки.
     * instant_payout_amount: Опциональная сумма аванса, которую
     *                        исполнитель получит сразу после финансирования.
     *                        Если аванс не нужен, передайте 0.
     *
     * Ожидаемые аккаунты:
     * 1. [Signer] Аккаунт инициатора сделки.
     * 2. [Writable] Аккаунт для хранения состояния сделки (должен быть новым).
     * 3. [] Аккаунт второй стороны (клиента или исполнителя).
     * 4. [] Аккаунт арбитра.
     * 5. [] Аккаунт системной программы (System Program).
     */
    InitializeEscrow {
        amount: u64,
        instant_payout_amount: u64, // 0 если аванс не требуется
    },

    FundEscrow,

    RaiseDispute,

    ResolveDispute {
        resolution: Resolution,
    },
}
