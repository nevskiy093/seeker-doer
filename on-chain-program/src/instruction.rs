use borsh::{BorshDeserialize, BorshSerialize};

// Варианты решения спора арбитром
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Resolution {
    // Вернуть оставшиеся средства заказчику
    RefundToClient,
    // Выплатить оставшиеся средства исполнителю
    PayToContractor,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum EscrowInstruction {
    InitializeEscrow {
        amount: u64,
    },

    FundEscrow,

    ReleasePrepayment {
        amount: u64,
    },

    ConfirmWorkDone,

    /**
     * Заказчик или исполнитель переводят сделку в состояние спора.
     *
     * Ожидаемые аккаунты:
     * 1. [Signer] Аккаунт того, кто начинает спор (заказчик или исполнитель).
     * 2. [Writable] Аккаунт сделки.
     */
    RaiseDispute,

    /**
     * Арбитр разрешает спор.
     *
     * Ожидаемые аккаунты:
     * 1. [Signer] Аккаунт арбитра.
     * 2. [Writable] Аккаунт сделки.
     * 3. [Writable] Аккаунт заказчика (может получить средства).
     * 4. [Writable] Аккаунт исполнителя (может получить средства).
     */
    ResolveDispute {
        resolution: Resolution,
    },
}
