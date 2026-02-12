import React from 'react';
import { WalletContextProvider } from './WalletContextProvider';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { useWallet } from '@solana/wallet-adapter-react';
import { CreateDealForm } from './components/CreateDealForm'; // Импортируем нашу форму

const AppContent = () => {
    const { publicKey } = useWallet();

    return (
        <div style={{ padding: '20px' }}>
            <header style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                <h1>Solana Escrow</h1>
                <WalletMultiButton />
            </header>

            <main style={{ marginTop: '40px' }}>
                {!publicKey ? (
                    <p>Please connect your wallet to create or manage deals.</p>
                ) : (
                    <div>
                        <p>Wallet Connected: {publicKey.toBase58()}</p>
                        <hr style={{ margin: '20px 0' }} />
                        <CreateDealForm />
                    </div>
                )}
            </main>
        </div>
    );
};

function App() {
    return (
        <WalletContextProvider>
            <AppContent />
        </WalletContextProvider>
    );
}

export default App;
