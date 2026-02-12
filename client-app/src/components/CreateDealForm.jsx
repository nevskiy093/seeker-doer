import React, { useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { PublicKey } from '@solana/web3.js';

// ВАШ КЛЮЧ АРБИТРА
const ARBITRATOR_PUBLIC_KEY = new PublicKey('5jmwZ7WWjBiVmXBbBA1feqoHk1gM38rATWrNrKaT2TEf'); 

export const CreateDealForm = () => {
    const { publicKey, sendTransaction } = useWallet();
    const [contractor, setContractor] = useState('');
    const [amount, setAmount] = useState('');
    const [prepayment, setPrepayment] = useState(''); // Новое поле для предоплаты

    const [error, setError] = useState('');
    const [success, setSuccess] = useState('');

    const handleSubmit = async (event) => {
        event.preventDefault();
        setError('');
        setSuccess('');

        if (!publicKey) {
            setError('Wallet not connected!');
            return;
        }

        // --- Валидация входных данных ---
        let contractorPk, dealAmount, prepaymentPercent;
        try {
            contractorPk = new PublicKey(contractor);
        } catch (err) {
            setError('Invalid Contractor Public Key');
            return;
        }
        
        dealAmount = parseFloat(amount);
        if (isNaN(dealAmount) || dealAmount <= 0) {
            setError('Invalid Amount');
            return;
        }

        prepaymentPercent = parseFloat(prepayment);
        if (isNaN(prepaymentPercent) || prepaymentPercent < 0 || prepaymentPercent > 100) {
            setError('Prepayment percentage must be between 0 and 100');
            return;
        }

        console.log('Creating deal with:');
        console.log('  Client:', publicKey.toBase58());
        console.log('  Contractor:', contractorPk.toBase58());
        console.log('  Arbitrator:', ARBITRATOR_PUBLIC_KEY.toBase58());
        console.log('  Amount (SOL):', dealAmount);
        console.log('  Prepayment (%):', prepaymentPercent);

        // TODO: Здесь будет логика создания и отправки транзакции
        setSuccess(`Deal creation initiated! (Transaction logic to be implemented)`);

        // Очищаем форму
        setContractor('');
        setAmount('');
        setPrepayment('');
    };

    return (
        <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', maxWidth: '500px', gap: '10px' }}>
            <h3>Create a New Escrow Deal</h3>
            <input
                type="text"
                placeholder="Contractor's Public Key"
                value={contractor}
                onChange={(e) => setContractor(e.target.value)}
                required
            />
            <input
                type="number"
                placeholder="Total Amount (in SOL)"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                step="0.000000001"
                min="0"
                required
            />
            <input
                type="number"
                placeholder="Prepayment Percentage (%)"
                value={prepayment}
                onChange={(e) => setPrepayment(e.target.value)}
                max="100"
                min="0"
                required
            />
            <button type="submit" disabled={!publicKey}>
                Create Deal
            </button>
            {error && <p style={{ color: 'red' }}>{error}</p>}
            {success && <p style={{ color: 'green' }}>{success}</p>}
        </form>
    );
};
