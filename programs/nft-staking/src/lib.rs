mod state;
mod ins;
mod constants;
mod errors;

use anchor_lang::prelude::*;
use metaplex_token_metadata::state::Metadata;
use spl_token::instruction::AuthorityType::AccountOwner;
use anchor_spl::{
    token::{self, Transfer},
};
use solana_program::program::invoke_signed;
use mpl_token_metadata::instruction::{freeze_delegated_account, thaw_delegated_account};

use ins::*;
use constants::*;
use errors::*;
use state::*;

declare_id!("Gfoam73aJ33wjPk4cLBnE6JQZ62twXjuoQokqp7imBMr");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_global(
        ctx: Context<InitializeGlobal>, 
        _global_bump: u8, 
        global_name: String,
        nft_creator: Pubkey,
        reward_token_mint: Pubkey,
        trait_rates: Vec<u64>,
        trait_names: Vec<String>,
        normal_rate: u64,
        lock_durations: Vec<u8>,
        lock_rates: Vec<u64>,
        custodial: bool,
    ) -> Result<()> {
        let global_authority = &mut ctx.accounts.global_authority;
        global_authority.name = global_name;
        global_authority.admin = ctx.accounts.admin.key();
        global_authority.nft_creator = nft_creator;
        global_authority.reward_token_mint = reward_token_mint;
        global_authority.total_amount = 0;
        global_authority.trait_rates = trait_rates;
        global_authority.trait_names = trait_names;
        global_authority.normal_rate = normal_rate;
        global_authority.lock_durations = lock_durations;
        global_authority.lock_rates = lock_rates;
        global_authority.custodial = custodial;
        Ok(())
    }

    pub fn update_admin(
        ctx: Context<UpdateAdmin>,
        _global_bump: u8,
        new_admin: Pubkey,
    ) -> Result<()> {
        let global_authority = &mut ctx.accounts.global_authority;
        require!(
            ctx.accounts.admin.key() == global_authority.admin,
            StakingError::InvalidAdmin
        );
        global_authority.admin = new_admin;

        Ok(())
    }

    pub fn update_global(
        ctx: Context<UpdateGlobal>, 
        _global_bump: u8, 
        nft_creator: Pubkey,
        reward_token_mint: Pubkey,
        trait_rates: Vec<u64>,
        trait_names: Vec<String>,
        normal_rate: u64,
        lock_durations: Vec<u8>,
        lock_rates: Vec<u64>,
        custodial: bool,
    ) -> Result<()> {
        let global_authority = &mut ctx.accounts.global_authority;
        require!(
            ctx.accounts.admin.key() == global_authority.admin,
            StakingError::InvalidAdmin
        );
        global_authority.nft_creator = nft_creator;
        global_authority.reward_token_mint = reward_token_mint;
        global_authority.trait_rates = trait_rates;
        global_authority.trait_names = trait_names;
        global_authority.normal_rate = normal_rate;
        global_authority.lock_durations = lock_durations;
        global_authority.lock_rates = lock_rates;
        global_authority.custodial = custodial;
        Ok(())
    }

    pub fn initialize_fixed_pool(ctx: Context<InitializeFixedPool>) -> Result<()> {
        let mut fixed_pool = ctx.accounts.user_fixed_pool.load_init()?;
        fixed_pool.owner = ctx.accounts.owner.key();
        Ok(())
    }

    #[access_control(user(&ctx.accounts.user_fixed_pool, &ctx.accounts.owner))]
    pub fn stake_nft_to_fixed(
        ctx: Context<StakeNftToFixed>,
        _global_bump: u8,
        lock_period: u8,
        role: String,
        model: u64,
    ) -> Result<()> {
        let mint_metadata = &mut &ctx.accounts.mint_metadata;

        msg!("Metadata Account: {:?}", ctx.accounts.mint_metadata.key());
        let (metadata, _) = Pubkey::find_program_address(
            &[
                metaplex_token_metadata::state::PREFIX.as_bytes(),
                metaplex_token_metadata::id().as_ref(),
                ctx.accounts.nft_mint.key().as_ref(),
            ],
            &metaplex_token_metadata::id(),
        );
        require!(
            metadata == mint_metadata.key(),
            StakingError::InvaliedMetadata
        );

        let global_authority = &mut ctx.accounts.global_authority;

        // verify metadata is legit
        let nft_metadata = Metadata::from_account_info(mint_metadata)?;

        if let Some(creators) = nft_metadata.data.creators {
            let mut valid: u8 = 0;
            for creator in creators {
                if creator.address == global_authority.nft_creator && creator.verified == true
                {
                    valid = 1;
                    break;
                }
            }
            if valid != 1 {
                return Err(StakingError::InvalidCollection.into());
            }
        } else {
            return Err(StakingError::MetadataCreatorParseError.into());
        };


        let timestamp = Clock::get()?.unix_timestamp;
        let lock_time = timestamp + DAY * lock_period as i64;

        let mut rate: i64 = 0;
        if model == 1 {
            let index = global_authority.trait_names.iter().position(|x| x == role.as_str());
            if let Some(index) = index {
                rate = global_authority.trait_rates[index] as i64;
            }
        }
        if model == 2 {
            rate = global_authority.normal_rate as i64;
        }
        if model == 3 {
            let index = global_authority.lock_durations.iter().position(|x| *x == lock_period);
            if let Some(index) = index {
                rate = global_authority.lock_rates[index] as i64
            }
        }
        let staked_item = StakedNFT {
            nft_addr: ctx.accounts.nft_mint.key(),
            stake_time: timestamp,
            reward_time: timestamp,
            lock_time: lock_time,
            rate: rate,
            model: model,
        };
        let mut fixed_pool = ctx.accounts.user_fixed_pool.load_mut()?;
        fixed_pool.add_nft(staked_item);

        ctx.accounts.global_authority.total_amount += 1;
        let token_account_info = &mut &ctx.accounts.user_token_account;
        
        let (vault_pda, vault_stake_bump) = Pubkey::find_program_address(
            &[
                VAULT_STAKE_SEED.as_bytes(),
                ctx.accounts.global_authority.key().as_ref(),
                ctx.accounts.owner.key().as_ref(),
                token_account_info.key().as_ref()
            ],
            ctx.program_id
        );

        let global_authority = ctx.accounts.global_authority.key().clone();
        let owner = ctx.accounts.owner.key().clone();
        let token_account_info = ctx.accounts.user_token_account.key().clone();

        let seeds = &[
            VAULT_STAKE_SEED.as_bytes(),
            global_authority.as_ref(),
            owner.as_ref(),
            token_account_info.as_ref(),
            &[vault_stake_bump],
        ];

        if ctx.accounts.global_authority.custodial == false {
            let cpi_context = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Approve {
                    to: ctx.accounts.user_token_account.to_account_info().clone(),
                    delegate: ctx.accounts.vault_pda.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info()
                }
            );
    
            anchor_spl::token::approve(cpi_context, 1)?;
            invoke_signed(
                &freeze_delegated_account(
                    ctx.accounts.token_metadata_program.key(),
                    ctx.accounts.vault_pda.key(),
                    token_account_info.key(),
                    ctx.accounts.edition.key(),
                    ctx.accounts.nft_mint.key(),
                ),
                &[
                    ctx.accounts.vault_pda.to_account_info(),
                    ctx.accounts.user_token_account.to_account_info(),
                    ctx.accounts.edition.to_account_info(),
                    ctx.accounts.nft_mint.to_account_info()
                ],
                &[seeds]
            )?;
        } else {
            let cpi_context = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::SetAuthority {
                    current_authority: ctx.accounts.owner.to_account_info().clone(),
                    account_or_mint: ctx.accounts.user_token_account.to_account_info().clone(),
                },
            );
            
            anchor_spl::token::set_authority(cpi_context, AccountOwner, Some(vault_pda))?;
        }
        
        Ok(())
    }

    #[access_control(user(&ctx.accounts.user_fixed_pool, &ctx.accounts.owner))]
    pub fn withdraw_nft_from_fixed(
        ctx: Context<WithdrawNftFromFixed>,
        _global_bump: u8,
        vault_stake_bump: u8,
    ) -> Result<()> {
        let timestamp = Clock::get()?.unix_timestamp;
        let mut fixed_pool = ctx.accounts.user_fixed_pool.load_mut()?;
        let reward: u64 = fixed_pool.remove_nft(
            ctx.accounts.owner.key(),
            ctx.accounts.nft_mint.key(),
            timestamp,
        )?;

        fixed_pool.pending_reward += reward;

        ctx.accounts.global_authority.total_amount -= 1;

        let global_authority = ctx.accounts.global_authority.key().clone();
        let owner = ctx.accounts.owner.key().clone();
        let token_account_info = ctx.accounts.user_token_account.key().clone();

        let seeds = &[
            VAULT_STAKE_SEED.as_bytes(),
            global_authority.as_ref(),
            owner.as_ref(),
            token_account_info.as_ref(),
            &[vault_stake_bump],
        ];

        if ctx.accounts.global_authority.custodial == false {
            invoke_signed(
                &thaw_delegated_account(
                    ctx.accounts.token_metadata_program.key(),
                    ctx.accounts.vault_pda.key(),
                    token_account_info.key(),
                    ctx.accounts.edition.key(),
                    ctx.accounts.nft_mint.key(),
                ),
                &[
                    ctx.accounts.vault_pda.to_account_info(),
                    ctx.accounts.user_token_account.to_account_info(),
                    ctx.accounts.edition.to_account_info(),
                    ctx.accounts.nft_mint.to_account_info()
                ],
                &[seeds]
            )?;
    
            let cpi_context = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Revoke {
                    source: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info()
                }
            );
    
            anchor_spl::token::revoke(cpi_context)?;
        } else {
            let cpi_context = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::SetAuthority {
                  current_authority: ctx.accounts.vault_pda.to_account_info().clone(),
                  account_or_mint: ctx.accounts.user_token_account.to_account_info().clone(),
                },
              );
            
            anchor_spl::token::set_authority(
                cpi_context.with_signer(&[&seeds[..]]),
                AccountOwner,
                Some(ctx.accounts.owner.key()), 
            )?;
        }        

        Ok(())
    }

    #[access_control(user(&ctx.accounts.user_fixed_pool, &ctx.accounts.owner))]
    pub fn claim_reward_all(ctx: Context<ClaimRewardAll>, global_bump: u8) -> Result<()> {
        let timestamp = Clock::get()?.unix_timestamp;
        let mut fixed_pool = ctx.accounts.user_fixed_pool.load_mut()?;
        let reward: u64 = fixed_pool.claim_reward_all(timestamp)?;
        msg!("Reward: {}", reward);
        if ctx.accounts.reward_vault.amount < reward {
            return Err(StakingError::LackLamports.into());
        }
        let global_authority = &ctx.accounts.global_authority;
        let name = global_authority.name.as_bytes();
        let seeds = &[
            name,
            GLOBAL_AUTHORITY_SEED.as_bytes(), 
            &[global_bump]
        ];
        let signer = &[&seeds[..]];
        let token_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.reward_vault.to_account_info(),
            to: ctx.accounts.user_reward_account.to_account_info(),
            authority: ctx.accounts.global_authority.to_account_info(),
        };
        token::transfer(
            CpiContext::new_with_signer(token_program.clone(), cpi_accounts, signer),
            reward,
        )?;

        Ok(())
    }

    #[access_control(user(&ctx.accounts.user_fixed_pool, &ctx.accounts.owner))]
    pub fn claim_reward(ctx: Context<ClaimReward>, global_bump: u8) -> Result<()> {
        let timestamp = Clock::get()?.unix_timestamp;
        let mut fixed_pool = ctx.accounts.user_fixed_pool.load_mut()?;
        let reward: u64 = fixed_pool.claim_reward(
            ctx.accounts.owner.key(),
            ctx.accounts.nft_mint.key(),
            timestamp,
        )?;
        msg!("Reward: {}", reward);
        if ctx.accounts.reward_vault.amount < reward {
            return Err(StakingError::LackLamports.into());
        }
        let global_authority = &ctx.accounts.global_authority;
        let name = global_authority.name.as_bytes();
        let seeds = &[
            name,
            GLOBAL_AUTHORITY_SEED.as_bytes(), 
            &[global_bump]
        ];
        let signer = &[&seeds[..]];
        let token_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.reward_vault.to_account_info(),
            to: ctx.accounts.user_reward_account.to_account_info(),
            authority: ctx.accounts.global_authority.to_account_info(),
        };
        token::transfer(
            CpiContext::new_with_signer(token_program.clone(), cpi_accounts, signer),
            reward,
        )?;

        Ok(())
    }

    pub fn withdraw_reward(ctx: Context<WithdrawReward>, global_bump: u8, amount: u64) -> Result<()> {
        let global_authority = &ctx.accounts.global_authority;
        let name = global_authority.name.as_bytes();
        let seeds = &[
            name,
            GLOBAL_AUTHORITY_SEED.as_bytes(), 
            &[global_bump]
        ];
        let signer = &[&seeds[..]];
        let token_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.reward_vault.to_account_info(),
            to: ctx.accounts.claimer_reward_account.to_account_info(),
            authority: ctx.accounts.global_authority.to_account_info(),
        };
        token::transfer(
            CpiContext::new_with_signer(token_program.clone(), cpi_accounts, signer),
            amount,
        )?;

        Ok(())
    }

    pub fn close_user_fixed_pool(ctx: Context<CloseUserFixedPool>) -> Result<()> {
        let fixed_pool = ctx.accounts.user_fixed_pool.load()?;
        if fixed_pool.item_count == 0 {
            let dest_account_info = ctx.accounts.owner.to_account_info();
            let source_account_info = ctx.accounts.user_fixed_pool.to_account_info();
            let dest_starting_lamports = dest_account_info.lamports();
            **dest_account_info.lamports.borrow_mut() = dest_starting_lamports
                .checked_add(source_account_info.lamports())
                .unwrap();
            **source_account_info.lamports.borrow_mut() = 0;
            // source_account_info.fill(0);
        }
        Ok(())
    }
}

// Access control modifiers
fn user(pool_loader: &AccountLoader<UserPool>, user: &AccountInfo) -> Result<()> {
    let user_pool = pool_loader.load()?;
    require!(user_pool.owner == *user.key, StakingError::InvalidUserPool);
    Ok(())
}
