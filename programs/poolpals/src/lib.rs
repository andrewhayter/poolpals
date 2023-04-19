use anchor_lang::prelude::*;
use anchor_lang::system_program;

// pub mod utils;
// pub use utils::*;

const LAMPORTS_PER_SOL: u64 = 1_000_000_000;
const LOTTERY_STATE_SEED: &[u8] = b"lottery_state";
const LOTTERY_VAULT_SEED: &[u8] = b"lottery_vault";
const USER_TICKET_DATA_SEED: &[u8] = b"user_ticket_data";

declare_id!("A4fkmt1rkdVbcAgAXR3g9d7VYfFdwm6kAQRz6zkk35UY");

#[account]
#[derive(InitSpace, Debug, PartialEq, Copy)]
pub struct LotteryState {
    authority: Pubkey,
    lottery_id: u64,
    status: LotteryStatus,
    ticket_price: u64,
    prize_pool: u64,
    ticket_count: u64,
    winner: Option<Pubkey>,
    start_time: i64,
    end_time: i64,
    max_tickets: u64,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct LotteryVault {
    lottery_id: u64,
    authority: Pubkey,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct TicketData {
    lottery_id: u64,
    user: Pubkey,
    ticket_count: u64,
    total_deposited: u64,
    bump: u8,
}

#[derive(InitSpace, Debug, AnchorSerialize, AnchorDeserialize, PartialEq, Clone, Copy)]
pub enum LotteryStatus {
    Pending,
    InProgress,
    Ended,
    Completed,
}

#[derive(Accounts)]
#[instruction(lottery_id: u64)]
pub struct InitializeLottery<'info> {
    #[account(init, payer = authority, space = 8 + LotteryState::INIT_SPACE, seeds = [LOTTERY_STATE_SEED, lottery_id.to_le_bytes().as_ref()], bump)]
    pub lottery_state: Account<'info, LotteryState>,
    #[account(init, payer = authority, space = 8 + LotteryVault::INIT_SPACE, seeds = [LOTTERY_VAULT_SEED, lottery_id.to_le_bytes().as_ref()], bump)]
    pub lottery_vault: Account<'info, LotteryVault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(deposit_amount: u64)]
pub struct BuyTickets<'info> {
    #[account(mut)]
    pub lottery_state: Account<'info, LotteryState>,
    #[account(mut)]
    pub lottery_vault: Account<'info, LotteryVault>,
    #[account(
        seeds = [
            USER_TICKET_DATA_SEED,
            lottery_state.lottery_id.to_le_bytes().as_ref(),
            user.key().as_ref()
        ],
        bump,
        init_if_needed,
        payer = user,
        space = 8 + TicketData::INIT_SPACE
    )]
    pub ticket_data: Account<'info, TicketData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[program]
pub mod poolpals {
    use super::*;

    pub fn initialize_lottery_state(
        ctx: Context<InitializeLottery>,
        lottery_id: u64,
        start_time: i64,
        end_time: i64,
        max_tickets: u64,
        ticket_price: u64,
    ) -> Result<()> {
        // Initialize the lottery state account
        let lottery_state = &mut ctx.accounts.lottery_state;

        lottery_state.authority = *ctx.accounts.authority.key;
        lottery_state.lottery_id = lottery_id;
        lottery_state.status = LotteryStatus::Pending;
        lottery_state.start_time = start_time;
        lottery_state.end_time = end_time;
        lottery_state.max_tickets = max_tickets;
        lottery_state.ticket_price = ticket_price;
        lottery_state.prize_pool = 0;
        lottery_state.ticket_count = 0;
        lottery_state.winner = None;

        // Initialize the lottery vault account
        let lottery_vault = &mut ctx.accounts.lottery_vault;
        lottery_vault.lottery_id = lottery_id;
        lottery_vault.authority = *ctx.accounts.authority.key;

        Ok(())
    }

    pub fn buy_tickets(ctx: Context<BuyTickets>, deposit: u64) -> Result<()> {
        let lottery_state = &mut ctx.accounts.lottery_state;
        let ticket_data = &mut ctx.accounts.ticket_data;

        if ticket_data.lottery_id == 0 {
            ticket_data.lottery_id = lottery_state.lottery_id;
            ticket_data.user = *ctx.accounts.user.key;
            ticket_data.ticket_count = 0;
        }

        if ticket_data.lottery_id != lottery_state.lottery_id {
            return Err(LotteryError::InvalidLotteryId.into());
        }

        // // if lottery_state.status != LotteryStatus::InProgress {
        // //     return Err(LotteryError::LotteryNotInProgress.into());
        // // }

        // if deposit % lottery_state.ticket_price != 0 {
        //     return Err(LotteryError::InvalidDeposit.into());
        // }

        let user_balance = ctx.accounts.user.lamports();
        if user_balance < deposit {
            return Err(LotteryError::InsufficientFunds.into());
        }

        let ticket_count = deposit / lottery_state.ticket_price;
        if lottery_state.ticket_count + ticket_count > lottery_state.max_tickets {
            return Err(LotteryError::InsufficientRemainingTickets.into());
        }

        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info().clone(),
                to: ctx.accounts.lottery_vault.to_account_info().clone(),
            },
        );
        system_program::transfer(cpi_context, deposit)?;

        ticket_data.ticket_count = ticket_data
            .ticket_count
            .checked_add(ticket_count)
            .ok_or(LotteryError::Overflow)?;

        ticket_data.total_deposited = ticket_data
            .total_deposited
            .checked_add(deposit * LAMPORTS_PER_SOL)
            .ok_or(LotteryError::Overflow)?;

        lottery_state.ticket_count = lottery_state
            .ticket_count
            .checked_add(ticket_count)
            .ok_or(LotteryError::Overflow)?;

        lottery_state.prize_pool = lottery_state
            .prize_pool
            .checked_add(deposit)
            .ok_or(LotteryError::Overflow)?;

        Ok(())
    }
}

#[error_code]
pub enum LotteryError {
    #[msg("Invalid lottery ID.")]
    InvalidLotteryId,
    #[msg("Lottery is not in progress.")]
    LotteryNotInProgress,
    #[msg("Invalid deposit.")]
    InvalidDeposit,
    #[msg("Insufficient funds.")]
    InsufficientFunds,
    #[msg("Overflow.")]
    Overflow,
    #[msg("Vault already initialized.")]
    VaultAlreadyInitialized,
    #[msg("Insufficient remaining tickets.")]
    InsufficientRemainingTickets,
    #[msg("Unauthorized.")]
    Unauthorized,
    #[msg("Invalid status transition.")]
    InvalidStatusTransition,
    #[msg("The lottery has not ended yet")]
    LotteryNotEnded,
    #[msg("There are no tickets purchased")]
    NoTickets,
    #[msg("No winner was selected")]
    NoWinner,
}
