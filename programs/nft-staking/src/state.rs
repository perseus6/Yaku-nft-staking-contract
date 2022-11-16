use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;

#[account]
#[derive(Default)]
pub struct GlobalPool {
    pub name: String,
    pub admin: Pubkey,
    pub nft_creator: Pubkey,
    pub reward_token_mint: Pubkey,
    pub total_amount: u64,
    pub trait_rates: Vec<u64>,
    pub trait_names: Vec<String>,
    pub normal_rate: u64,
    pub lock_durations: Vec<u8>,
    pub lock_rates: Vec<u64>,
    pub custodial: bool,
}

impl GlobalPool {
  pub const LEN: usize = (8 + 10) + 32 + 32 + 32 + 8 + (8 + 8 * 5) + (8 + (10 + 8) * 5) + 8 + (1 * 3 + 8) + (8 * 3 + 8) + 1;
}

#[zero_copy]
#[derive(Default, PartialEq)]
pub struct StakedNFT {
    pub nft_addr: Pubkey,
    pub stake_time: i64,
    pub reward_time: i64,
    pub lock_time: i64,
    pub rate: i64,
    pub model: u64,
}

#[account(zero_copy)]
pub struct UserPool {
    // 12064
    pub owner: Pubkey,                           // 32
    pub item_count: u64,                         // 8
    pub items: [StakedNFT; NFT_STAKE_MAX_COUNT], // (72 + 8) * 150 = 12000
    pub reward_time: i64,                        // 8
    pub pending_reward: u64,                     // 8
}

impl Default for UserPool {
  #[inline]
  fn default() -> UserPool {
      UserPool {
          owner: Pubkey::default(),
          item_count: 0,
          items: [StakedNFT {
              ..Default::default()
          }; NFT_STAKE_MAX_COUNT],
          reward_time: 0,
          pending_reward: 0,
      }
  }
}

impl UserPool {
    pub fn add_nft(&mut self, item: StakedNFT) {
        self.items[self.item_count as usize] = item;
        self.item_count += 1;
    }
    pub fn remove_nft(&mut self, owner: Pubkey, nft_mint: Pubkey, now: i64) -> Result<u64> {
        require!(self.owner.eq(&owner), StakingError::InvalidOwner);
        let mut withdrawn: u8 = 0;
        let mut reward: u64 = 0;
        for i in 0..self.item_count {
            let index = i as usize;
            if self.items[index].nft_addr.eq(&nft_mint) {
                if self.items[index].model == 3 {
                    require!(
                        self.items[index].lock_time < now,
                        StakingError::BeforeLockTime
                    );
                }
                let mut last_reward_time = self.reward_time;
                if last_reward_time < self.items[index].stake_time {
                    last_reward_time = self.items[index].stake_time;
                }

                reward = (self.items[index].rate * (now - last_reward_time) / DAY) as u64;

                // remove nft
                if i != self.item_count - 1 {
                    let last_idx = self.item_count - 1;
                    self.items[index] = self.items[last_idx as usize];
                }
                self.item_count -= 1;
                withdrawn = 1;
                break;
            }
        }
        require!(withdrawn == 1, StakingError::InvalidNFTAddress);
        Ok(reward)
    }
    pub fn claim_reward(&mut self, owner: Pubkey, nft_mint: Pubkey, now: i64) -> Result<u64> {
        require!(self.owner.eq(&owner), StakingError::InvalidOwner);
        let mut reward: u64 = 0;
        for i in 0..self.item_count {
            let index = i as usize;
            if self.items[index].nft_addr.eq(&nft_mint) {
                let mut last_reward_time = self.items[index].reward_time;
                if last_reward_time < self.items[index].stake_time {
                    last_reward_time = self.items[index].stake_time;
                }
                
                reward = (self.items[index].rate * (now - last_reward_time) / DAY) as u64;
                self.items[index].reward_time = now;
            }
        }
        Ok(reward)
    }

    pub fn claim_reward_all(&mut self, now: i64) -> Result<u64> {
        let mut total_reward: u64 = 0;
        for i in 0..self.item_count {
            let index = i as usize;
            let mut last_reward_time = self.items[index].reward_time;
            if last_reward_time < self.items[index].stake_time {
                last_reward_time = self.items[index].stake_time;
            }
            let reward = (self.items[index].rate * (now - last_reward_time) / DAY) as u64;
            total_reward += reward;
            self.items[index].reward_time = now; //Super added this
        }
        total_reward += self.pending_reward;
        self.pending_reward = 0;
        self.reward_time = now;
        Ok(total_reward)
    }
}