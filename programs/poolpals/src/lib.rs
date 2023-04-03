// mod error;
use anchor_lang::prelude::*;
use anchor_spl::token;

pub mod error;

use crate::error::ErrorCode;

const AUTHORIZED_INITIATOR: &str = "Dvfu3r2iWSRri8bWdwwLQ5Krujogf3ZRauVRSXYRKAwN";

declare_id!("FCy8iFxZmzpdqFhDh8nMEt5maHSCYPijAPqJonChehjJ");

#[program]
pub mod poolpals {
    use super::*;

    pub fn initialize(ctx: Context<InitializeLottery>, authority: Pubkey) -> Result<()> {
        // Check if the provided authority matches the authorized initiator's public key
        let authorized_initiator = Pubkey::try_from(AUTHORIZED_INITIATOR)
            .map_err(|_| ErrorCode::InvalidAuthorizedInitiator)?;
        if authority != authorized_initiator {
            return Err(ErrorCode::UnauthorizedInitiator.into());
        }

        let lottery = &mut ctx.accounts.lottery;

        lottery.is_initalized = true;
        lottery.is_finalized = false;
        lottery.lottery_id = 0;
        lottery.total_entires = 0;
        lottery.winning_number = 0;
        lottery.bump = *ctx.bumps.get("lottery").unwrap();

        msg!("{:?}", lottery);

        Ok(())
    }

    // pub fn stake_tokens(ctx: Context<StakeTokens>) -> Result<()> {
    //     //

    //     Ok(())
    // }

    // pub fn store_winning_numbers(ctx: Context<StoreWinningNumbers>) -> Result<()> {
    //     //

    //     Ok(())
    // }

    // pub fn reward_winners(ctx: Context<RewardWinners>) -> Result<()> {
    //     //

    //     Ok(())
    // }
}

// accounts

#[derive(Accounts)]
pub struct InitializeLottery<'info> {
    #[account(
      init,
      payer = initializer,
      space = 8 + Lottery::INIT_SPACE,
      constraint = !lottery.is_initalized @ ProgramError::AccountAlreadyInitialized
    )]
    pub lottery: Account<'info, Lottery>,
    #[account(
      init,
      payer = initializer,
      space = 8 + GlobalStake::INIT_SPACE, seeds = [b"global_stake"], bump)]
    pub global_stake: Account<'info, GlobalStake>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct StakeTokens<'info> {
//     #[account(mut)]
//     pub lottery: Account<'info, Lottery>,
//     //global stake account
//     #[account(mut)]
//     /// CHECK:
//     pub global_stake: AccountInfo<'info>,
//     #[account(init, seeds = [b"user_stake", payer.key().as_ref()], bump, payer = payer, space = 8 + 1+32+8)]
//     //user stake account
//     /// CHECK:
//     pub user_stake: AccountInfo<'info>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     /// CHECK:
//     pub system_program: Program<'info, System>,
// }

// state

#[account]
#[derive(InitSpace, Debug)]
pub struct Lottery {
    pub bump: u8,
    pub is_initalized: bool,
    pub is_finalized: bool,
    pub lottery_id: u32,
    pub total_entires: u32,
    pub winning_number: u64,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct GlobalStake {
    pub stake_mint: Option<Pubkey>,
    pub stake_amount: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct UserStake {
    pub stake_mint: Option<Pubkey>,
    pub stake_amount: u64,
}
