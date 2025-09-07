import React, { useState, useEffect, useMemo } from 'react';
import { Link, useLocation } from 'react-router-dom';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { useWallet } from '@solana/wallet-adapter-react';
import { useConnection } from '@solana/wallet-adapter-react';
import { Connection, PublicKey } from '@solana/web3.js';
import { AnchorProvider, Program } from '@project-serum/anchor';
import { web3 } from '@project-serum/anchor';
import { useToken } from '../../hooks/useToken';
import { useTokenAccount } from '../../hooks/useTokenAccount';
import { useTokenMint } from '../../hooks/useTokenMint';
import { useTokenSupply } from '../../hooks/useTokenSupply';
import { useTokenMetadata } from '../../hooks/useTokenMetadata';

const Navbar = () => {
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const { publicKey, wallet } = useWallet();
  const { connection } = useConnection();
  const [provider, setProvider] = useState(null);
  const [program, setProgram] = useState(null);
  const [tokenAccounts, setTokenAccounts] = useState([]);
  const [tokenMints, setTokenMints] = useState([]);
  const [tokenSupplies, setTokenSupplies] = useState([]);
  const [tokenMetadatas, setTokenMetadatas] = useState([]);

  const location = useLocation();

  useEffect(() => {
    const initProvider = async () => {
      if (publicKey && wallet) {
        const provider = new AnchorProvider(
          connection,
          {
            publicKey: publicKey,
            signTransaction: wallet.adapter.signTransaction,
            signAllTransactions: wallet.adapter.signAllTransactions,
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
  }, [publicKey, wallet, connection]);

  useEffect(() => {
    const fetchTokenData = async () => {
      if (!publicKey || !program) return;

      try {
        const tokenAccounts = await useTokenAccount(publicKey, connection);
        setTokenAccounts(tokenAccounts);

        const tokenMints = await Promise.all(tokenAccounts.map(account => useTokenMint(account.mint, connection)));
        setTokenMints(tokenMints);

        const tokenSupplies = await Promise.all(tokenMints.map(mint => useTokenSupply(mint, connection)));
        setTokenSupplies(tokenSupplies);

        const tokenMetadatas = await Promise.all(tokenMints.map(mint => useTokenMetadata(mint, connection)));
        setTokenMetadatas(tokenMetadatas);
      } catch (error) {
        console.error('Error fetching token data:', error);
      }
    };

    fetchTokenData();
  }, [publicKey, program, connection]);

  const toggleMenu = () => {
    setIsMenuOpen(!isMenuOpen);
  };

  const closeMenu = () => {
    setIsMenuOpen(false);
  };

  const navLinks = useMemo(() => [
    { path: '/', label: 'Home' },
    { path: '/token-management', label: 'Token Management' },
    { path: '/token-transfer', label: 'Token Transfer' },
    { path: '/token-burn', label: 'Token Burn' },
    { path: '/token-freeze', label: 'Token Freeze' },
    { path: '/token-thaw', label: 'Token Thaw' },
    { path: '/token-supply', label: 'Token Supply' },
    { path: '/token-approve', label: 'Token Approve' },
    { path: '/token-revoke', label: 'Token Revoke' },
    { path: '/token-close-account', label: 'Token Close Account' },
    { path: '/token-set-authority', label: 'Token Set Authority' },
    { path: '/token-account-info', label: 'Token Account Info' },
    { path: '/token-mint-info', label: 'Token Mint Info' },
    { path: '/token-supply-info', label: 'Token Supply Info' },
    { path: '/token-accounts-by-owner', label: 'Token Accounts by Owner' },
    { path: '/token-accounts-by-delegate', label: 'Token Accounts by Delegate' },
    { path: '/token-accounts-by-mint', label: 'Token Accounts by Mint' },
    { path: '/token-accounts-by-program-id', label: 'Token Accounts by Program ID' },
    { path: '/token-accounts-by-associated-token-address', label: 'Token Accounts by Associated Token Address' },
  ], []);

  return (
    <nav className="bg-gray-800 p-4">
      <div className="container mx-auto flex justify-between items-center">
        <Link to="/" className="text-white text-2xl font-bold">
          TokenCraft
        </Link>
        <div className="hidden md:flex space-x-4">
          {navLinks.map((link, index) => (
            <Link
              key={index}
              to={link.path}
              className={`text-white hover:text-gray-300 ${location.pathname === link.path ? 'font-bold' : ''}`}
              onClick={closeMenu}
            >
              {link.label}
            </Link>
          ))}
        </div>
        <div className="md:hidden">
          <button
            onClick={toggleMenu}
            className="text-white focus:outline-none"
          >
            <svg
              className="h-6 w-6 fill-current"
              viewBox="0 0 24 24"
              xmlns="http://www.w3.org/2000/svg"
            >
              {isMenuOpen ? (
                <path
                  fillRule="evenodd"
                  clipRule="evenodd"
                  d="M4 6h16v2H4V6zm0 5h16v2H4v-2zm16 5H4v2h16v-2z"
                />
              ) : (
                <path
                  fillRule="evenodd"
                  clipRule="evenodd"
                  d="M4 6h16v2H4V6zm0 5h16v2H4v-2zm16 5H4v2h16v-2z"
                />
              )}
            </svg>
          </button>
        </div>
        <div className="ml-4">
          <WalletMultiButton />
        </div>
      </div>
      {isMenuOpen && (
        <div className="md:hidden bg-gray-700 mt-2 p-4">
          {navLinks.map((link, index) => (
            <Link
              key={index}
              to={link.path}
              className={`block text-white py-2 hover:text-gray-300 ${location.pathname === link.path ? 'font-bold' : ''}`}
              onClick={closeMenu}
            >
              {link.label}
            </Link>
          ))}
        </div>
      )}
    </nav>
  );
};

export default Navbar;