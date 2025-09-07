import { useState, useEffect, useCallback } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { Connection, PublicKey } from '@solana/web3.js';
import { AnchorProvider, Program } from '@project-serum/anchor';

const useWallet = () => {
  const { publicKey, wallet, sendTransaction, signTransaction, signAllTransactions } = useWallet();
  const [provider, setProvider] = useState(null);
  const [program, setProgram] = useState(null);
  const [connection, setConnection] = useState(new Connection('https://api.devnet.solana.com', 'confirmed'));

  useEffect(() => {
    const initProvider = async () => {
      if (publicKey && wallet) {
        const provider = new AnchorProvider(
          connection,
          {
            publicKey: publicKey,
            signTransaction: signTransaction,
            signAllTransactions: signAllTransactions,
          },
          {
            preflightCommitment: 'confirmed',
            commitment: 'confirmed',
          }
        );
        setProvider(provider);

        const idl = await Program.fetchIdl(new PublicKey('YourProgramIDHere'), provider);
        const program = new Program(idl, new PublicKey('YourProgramIDHere'), provider);
        setProgram(program);
      }
    };
    initProvider();
  }, [publicKey, wallet, connection, signTransaction, signAllTransactions]);

  const connectWallet = useCallback(async () => {
    if (wallet) {
      try {
        await wallet.adapter.connect();
      } catch (error) {
        console.error('Error connecting wallet:', error);
      }
    }
  }, [wallet]);

  const disconnectWallet = useCallback(() => {
    if (wallet) {
      wallet.adapter.disconnect();
    }
  }, [wallet]);

  const sendSolanaTransaction = useCallback(async (transaction) => {
    if (!publicKey || !wallet) {
      throw new Error('Wallet not connected');
    }
    try {
      const signature = await sendTransaction(transaction, connection);
      await connection.confirmTransaction(signature, 'confirmed');
      return signature;
    } catch (error) {
      console.error('Error sending transaction:', error);
      throw error;
    }
  }, [publicKey, wallet, sendTransaction, connection]);

  return {
    publicKey,
    wallet,
    provider,
    program,
    connection,
    connectWallet,
    disconnectWallet,
    sendSolanaTransaction,
  };
};

export default useWallet;