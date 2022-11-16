use anchor_lang::prelude::*;
use anchor_spl::{
  token::{Token, TokenAccount},
};

use crate::state::*;
use crate::constants::*;


#[derive(Accounts)]
#[instruction(global_bump: u8, global_name: String)]
pub struct InitializeGlobal<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [
          global_name.as_ref(),
          GLOBAL_AUTHORITY_SEED.as_ref(),
        ],
        bump,
        payer = admin,
        space = GlobalPool::LEN + 8
    )]
    pub global_authority: Account<'info, GlobalPool>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(global_bump: u8)]
pub struct UpdateAdmin<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [
          global_authority.name.as_ref(),
          GLOBAL_AUTHORITY_SEED.as_ref()
        ],
        bump = global_bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,
}

#[derive(Accounts)]
#[instruction(global_bump: u8)]
pub struct UpdateGlobal<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [
          global_authority.name.as_ref(),
          GLOBAL_AUTHORITY_SEED.as_ref()
        ],
        bump = global_bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,
}

#[derive(Accounts)]
pub struct InitializeFixedPool<'info> {
    #[account(zero)]
    pub user_fixed_pool: AccountLoader<'info, UserPool>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(global_bump: u8)]
pub struct StakeNftToFixed<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub user_fixed_pool: AccountLoader<'info, UserPool>,

    #[account(
        mut,
        seeds = [
          global_authority.name.as_ref(),
          GLOBAL_AUTHORITY_SEED.as_ref()
        ],
        bump = global_bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,
    #[account(
        mut,
        constraint = user_token_account.mint == *nft_mint.to_account_info().key,
        constraint = user_token_account.owner == *owner.key,
        constraint = user_token_account.amount == 1,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    /// CHECK:
    pub nft_mint: AccountInfo<'info>,
    
    /// CHECK:
    #[account(mut)]
    pub vault_pda: AccountInfo<'info>,

    /// CHECK:
    pub edition: AccountInfo<'info>,

    /// CHECK:
    #[account(
        mut,
        constraint = mint_metadata.owner == &metaplex_token_metadata::ID
    )]
    pub mint_metadata: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,

    // the token metadata program
    /// CHECK:
    #[account(constraint = token_metadata_program.key == &metaplex_token_metadata::ID)]
    pub token_metadata_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(global_bump: u8, vault_stake_bump: u8)]
pub struct WithdrawNftFromFixed<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub user_fixed_pool: AccountLoader<'info, UserPool>,

    #[account(
        mut,
        seeds = [
          global_authority.name.as_ref(),
          GLOBAL_AUTHORITY_SEED.as_ref()
        ],
        bump = global_bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,

    /// CHECK:
    #[account(
        mut,
        seeds = [
            VAULT_STAKE_SEED.as_bytes(), 
            global_authority.key().as_ref(), 
            owner.key().as_ref(),
            user_token_account.key().as_ref(),
        ],
        bump = vault_stake_bump,
    )]
    pub vault_pda: AccountInfo<'info>,
    /// CHECK:
    pub edition: AccountInfo<'info>,

    #[account(
        mut,
        constraint = user_token_account.mint == *nft_mint.to_account_info().key,
        // constraint = user_token_account.owner == *owner.to_account_info().key,
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    /// CHECK:
    pub nft_mint: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    // the token metadata program
    /// CHECK:
    #[account(constraint = token_metadata_program.key == &metaplex_token_metadata::ID)]
    pub token_metadata_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(global_bump: u8)]
pub struct ClaimRewardAll<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub user_fixed_pool: AccountLoader<'info, UserPool>,

    #[account(
        mut,
        seeds = [
          global_authority.name.as_ref(),
          GLOBAL_AUTHORITY_SEED.as_ref()
        ],
        bump = global_bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,

    #[account(
        mut,
        constraint = reward_vault.mint == global_authority.reward_token_mint,
        constraint = reward_vault.owner == global_authority.key(),
    )]
    pub reward_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = user_reward_account.mint == global_authority.reward_token_mint,
        constraint = user_reward_account.owner == owner.key(),
    )]
    pub user_reward_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(global_bump: u8)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub user_fixed_pool: AccountLoader<'info, UserPool>,

    #[account(
        mut,
        seeds = [
          global_authority.name.as_ref(),
          GLOBAL_AUTHORITY_SEED.as_ref()
        ],
        bump = global_bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,

    #[account(
        mut,
        constraint = reward_vault.mint == global_authority.reward_token_mint,
        constraint = reward_vault.owner == global_authority.key(),
    )]
    pub reward_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = user_reward_account.mint == global_authority.reward_token_mint,
        constraint = user_reward_account.owner == owner.key(),
    )]
    pub user_reward_account: Account<'info, TokenAccount>,

    /// CHECK:
    pub nft_mint: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(global_bump: u8)]
pub struct WithdrawReward<'info>
{
    #[account(mut)]
    pub claimer: Signer<'info>,

    #[account(
        mut,
        seeds = [
          global_authority.name.as_ref(),
          GLOBAL_AUTHORITY_SEED.as_ref()
        ],
        bump = global_bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,

    #[account(
        mut,
        constraint = reward_vault.mint == global_authority.reward_token_mint,
        constraint = reward_vault.owner == global_authority.key(),
    )]
    pub reward_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = claimer_reward_account.mint == global_authority.reward_token_mint,
        constraint = claimer_reward_account.owner == claimer.key(),
    )]
    pub claimer_reward_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CloseUserFixedPool<'info> {
    #[account(mut)]
    pub owner: SystemAccount<'info>,

    #[account(mut)]
    pub user_fixed_pool: AccountLoader<'info, UserPool>,
}