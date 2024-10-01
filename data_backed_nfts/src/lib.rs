use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use solana_program::clock::Clock;
use anchor_spl::token_interface::accessor::mint;
// use crate::token::accessor::mint;
    

declare_id!("GSCRHAmMxSWTh56bAqpFRMqHn9PS1zBHiMxsHiej1cWR");

#[program]
mod data_backed_nfts {
    use super::*;

    // Initializes the Data Account and assigns dataset info to the NFT
    pub fn initialize(ctx: Context<Initialize>, dataset_id: String, metadata_url: String, bump: u8, expiry: i64) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        data_account.dataset_id = dataset_id;
        data_account.metadata_url = metadata_url;
        data_account.authority = ctx.accounts.mint_authority.key();
        data_account.bump = bump;
        data_account.expiry = expiry; // Expiry timestamp for leasing or access control
        Ok(())
    }

    // Allows NFT holders to access the dataset associated with the NFT
    pub fn grant_access(ctx: Context<GrantAccess>) -> Result<()> {
        let nft_holder = &ctx.accounts.nft_holder;

        // Verify if the NFT holder has the required NFT token amount
        require!(nft_holder.amount > 0, CustomError::NoNFTHeld);

        let data_account = &ctx.accounts.data_account;
        
        // Check if access has expired
        let current_timestamp = Clock::get()?.unix_timestamp;
        require!(current_timestamp <= data_account.expiry, CustomError::AccessExpired);

        // Log access event
        emit!(AccessGranted {
            user: ctx.accounts.user.key(),
            dataset_id: data_account.dataset_id.clone(),
        });

        msg!("Access granted to dataset: {}, metadata: {}", data_account.dataset_id, data_account.metadata_url);

        Ok(())
    }

    // Allows NFT holders to extend their lease
    pub fn extend_lease(ctx: Context<ExtendLease>, additional_time: i64) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        let nft_holder = &ctx.accounts.nft_holder;

        // Ensure the NFT holder has the token
        require!(nft_holder.amount > 0, CustomError::NoNFTHeld);

        // Extend the lease time
        data_account.expiry += additional_time;

        msg!("Lease extended. New expiry: {}", data_account.expiry);

        Ok(())
    }

    // Allows the current authority to transfer ownership of the dataset to a new authority
    pub fn transfer_ownership(ctx: Context<TransferOwnership>, new_authority: Pubkey) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;

        // Ensure the current user is the authority
        require!(ctx.accounts.user.key() == data_account.authority, CustomError::Unauthorized);

        // Transfer ownership to the new authority
        data_account.authority = new_authority;

        msg!("Ownership transferred to {}", new_authority);

        Ok(())
    }

    // Grants access to dataset with a fee (SPL token transfer)
    pub fn grant_access_with_fee(ctx: Context<GrantAccessWithFee>, fee: u64) -> Result<()> {
        let nft_holder = &ctx.accounts.nft_holder;

        // Verify if the NFT holder has the required NFT token amount
        require!(nft_holder.amount > 0, CustomError::NoNFTHeld);

        let data_account = &ctx.accounts.data_account;
        
        // Check if access has expired
        let current_timestamp = Clock::get()?.unix_timestamp;
        require!(current_timestamp <= data_account.expiry, CustomError::AccessExpired);

        // Charge the access fee using a token transfer
        token::transfer(
            ctx.accounts.into_transfer_context(),
            fee,
        )?;

        msg!("Access granted to dataset: {}", data_account.dataset_id);

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(dataset_id: String, metadata_url: String, bump: u8, expiry: i64)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 64 + 256 + 8, // discriminator + Pubkey + dataset_id + metadata_url + expiry
        seeds = [b"data-access", nft_mint.key().as_ref()],
        bump
    )]
    pub data_account: Account<'info, DataAccount>,
    pub nft_mint: Account<'info, Mint>,
    pub mint_authority: Signer<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct GrantAccess<'info> {
    #[account(
        mut,
        seeds = [b"data-access", nft_mint.key().as_ref()],
        bump = data_account.bump
    )]
    pub data_account: Account<'info, DataAccount>,
    #[account(
        mut,
        constraint = nft_holder.mint == nft_mint.key(),
        constraint = nft_holder.owner == user.key()
    )]
    pub nft_holder: Account<'info, TokenAccount>,
    pub nft_mint: Account<'info, Mint>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ExtendLease<'info> {
    #[account(
        mut,
        seeds = [b"data-access", nft_mint.key().as_ref()],
        bump = data_account.bump
    )]
    pub data_account: Account<'info, DataAccount>,
    #[account(
        mut,
        constraint = nft_holder.mint == nft_mint.key(),
        constraint = nft_holder.owner == user.key()
    )]
    pub nft_holder: Account<'info, TokenAccount>,
    pub nft_mint: Account<'info, Mint>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(
        mut,
        seeds = [b"data-access", nft_mint.key().as_ref()],
        bump = data_account.bump
    )]
    pub data_account: Account<'info, DataAccount>,
    pub nft_mint: Account<'info, Mint>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct GrantAccessWithFee<'info> {
    #[account(
        mut,
        seeds = [b"data-access", nft_mint.key().as_ref()],
        bump = data_account.bump
    )]
    pub data_account: Account<'info, DataAccount>,
    #[account(
        mut,
        constraint = nft_holder.mint == nft_mint.key(),
        constraint = nft_holder.owner == user.key()
    )]
    pub nft_holder: Account<'info, TokenAccount>,
    pub nft_mint: Account<'info, Mint>,
    pub user: Signer<'info>,
    #[account(mut)]
    pub payment_destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> GrantAccessWithFee<'info> {
    fn into_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.nft_holder.to_account_info().clone(),
            to: self.payment_destination.to_account_info().clone(),
            authority: self.user.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }
}

#[account]
pub struct DataAccount {
    pub dataset_id: String,   // ID of the dataset associated with the NFT
    pub metadata_url: String, // URL pointing to the metadata (e.g., IPFS or Arweave)
    pub authority: Pubkey,    // Authority of the NFT mint
    pub bump: u8,             // PDA bump seed
    pub expiry: i64,          // Expiry timestamp for leasing or time-based access
}

#[event]
pub struct AccessGranted {
    pub user: Pubkey,
    pub dataset_id: String,
}

#[error_code]
pub enum CustomError {
    #[msg("User does not hold the required NFT.")]
    NoNFTHeld,
    #[msg("Access to this dataset has expired.")]
    AccessExpired,
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}
