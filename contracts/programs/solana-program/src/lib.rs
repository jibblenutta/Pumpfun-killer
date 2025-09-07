use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::{Metadata, MetadataAccount};

declare_id!("YourProgramIDHere");

#[program]
pub mod token_craft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, symbol: String, uri: String) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;
        let mint = &ctx.accounts.mint;
        let metadata_account = &mut ctx.accounts.metadata_account;
        let master_edition_account = &mut ctx.accounts.master_edition_account;

        // Initialize token account
        token_account.mint = mint.key();
        token_account.owner = ctx.accounts.owner.key();
        token_account.amount = 1_000_000_000;

        // Initialize metadata
        metadata_account.mint = mint.key();
        metadata_account.update_authority = ctx.accounts.owner.key();
        metadata_account.data = Metadata::new(
            name,
            symbol,
            uri,
            None,
            0,
            None,
        );
        metadata_account.is_mutable = true;

        // Initialize master edition
        master_edition_account.mint = mint.key();
        master_edition_account.update_authority = ctx.accounts.owner.key();
        master_edition_account.max_supply = Some(0);

        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        let from = &ctx.accounts.from;
        let to = &ctx.accounts.to;

        // Transfer tokens
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn burn(ctx: Context<Burn>, amount: u64) -> Result<()> {
        let token_account = &ctx.accounts.token_account;

        // Burn tokens
        anchor_spl::token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: token_account.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn freeze(ctx: Context<Freeze>) -> Result<()> {
        let token_account = &ctx.accounts.token_account;

        // Freeze token account
        anchor_spl::token::freeze_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::FreezeAccount {
                    account: token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.freeze_authority.to_account_info(),
                },
            ),
        )?;

        Ok(())
    }

    pub fn thaw(ctx: Context<Thaw>) -> Result<()> {
        let token_account = &ctx.accounts.token_account;

        // Thaw token account
        anchor_spl::token::thaw_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::ThawAccount {
                    account: token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.freeze_authority.to_account_info(),
                },
            ),
        )?;

        Ok(())
    }

    pub fn approve(ctx: Context<Approve>, amount: u64) -> Result<()> {
        let token_account = &ctx.accounts.token_account;
        let delegate = &ctx.accounts.delegate;

        // Approve delegate
        anchor_spl::token::approve(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Approve {
                    to: token_account.to_account_info(),
                    delegate: delegate.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn revoke(ctx: Context<Revoke>) -> Result<()> {
        let token_account = &ctx.accounts.token_account;

        // Revoke delegate
        anchor_spl::token::revoke(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Revoke {
                    source: token_account.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
        )?;

        Ok(())
    }

    pub fn close_account(ctx: Context<CloseAccount>) -> Result<()> {
        let token_account = &ctx.accounts.token_account;

        // Close token account
        anchor_spl::token::close_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::CloseAccount {
                    account: token_account.to_account_info(),
                    destination: ctx.accounts.owner.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
        )?;

        Ok(())
    }

    pub fn set_authority(ctx: Context<SetAuthority>, new_authority: Pubkey, authority_type: AuthorityType) -> Result<()> {
        let mint = &ctx.accounts.mint;

        // Set authority
        anchor_spl::token::set_authority(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::SetAuthority {
                    account_or_mint: mint.to_account_info(),
                    current_authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            authority_type,
            Some(new_authority),
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 32 + 8)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(init, payer = owner, mint::decimals = 9, mint::authority = mint_authority)]
    pub mint: Account<'info, Mint>,
    #[account(init, payer = owner, space = Metadata::LEN)]
    pub metadata_account: Account<'info, MetadataAccount>,
    #[account(init, payer = owner, space = anchor_spl::metadata::MasterEdition::LEN)]
    pub master_edition_account: Account<'info, anchor_spl::metadata::MasterEdition>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub metadata_program: Program<'info, Metadata>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Freeze<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub freeze_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Thaw<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub freeze_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub delegate: AccountInfo<'info>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Revoke<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SetAuthority<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct TokenAccount {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Invalid owner")]
    InvalidOwner,
    #[msg("Invalid delegate")]
    InvalidDelegate,
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Invalid metadata")]
    InvalidMetadata,
    #[msg("Invalid master edition")]
    InvalidMasterEdition,
}