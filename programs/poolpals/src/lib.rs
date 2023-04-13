use anchor_lang::prelude::*;

declare_id!("4Et6MKLER9MBDCoKeV8xG3Ei2nxGRTW3sWNYmUcjGdmZ");

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
#[derive(Debug, InitSpace)]
pub struct LotteryVault {
    lottery_id: u64,
    authority: Pubkey,
}

#[account]
#[derive(Debug, InitSpace)]
pub struct TicketData {
    lottery_id: u64,
    user: Pubkey,
    ticket_count: u64,
    total_deposited: u64,
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, PartialEq, Clone, Copy, InitSpace)]
pub enum LotteryStatus {
    Pending,
    InProgress,
    Ended,
    Completed,
}

#[derive(Accounts)]
#[instruction(lottery_id: u64, bump: u8)]
pub struct InitializeLottery<'info> {
    #[account(init, payer = authority, space = 8 + LotteryState::INIT_SPACE, seeds = [b"lottery_state", lottery_id.to_le_bytes().as_ref()], bump)]
    pub lottery_state: Account<'info, LotteryState>,
    #[account(init, payer = authority, space = 8 + LotteryVault::INIT_SPACE, seeds = [b"lottery_vault", lottery_id.to_le_bytes().as_ref()], bump)]
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
            b"user_ticket_data",
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
pub mod lottery {
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

        msg!("ctx.accounts.user.key: {}", ctx.accounts.user.key);
        msg!("deposit: {}", deposit);
        msg!("lottery_state: {:?}", lottery_state);
        msg!("before ticket_data: {:?}", ticket_data);

        // fix this
        if ticket_data.lottery_id == 0 {
            ticket_data.lottery_id = lottery_state.lottery_id;
            ticket_data.user = *ctx.accounts.user.key;
            ticket_data.ticket_count = 0;
        }

        msg!("after ticket_data: {:?}", ticket_data);

        // if ticket_data.lottery_id != lottery_state.lottery_id {
        //     return Err(LotteryError::InvalidLotteryId.into());
        // }

        // // if lottery_state.status != LotteryStatus::InProgress {
        // //     return Err(LotteryError::LotteryNotInProgress.into());
        // // }

        // if deposit % lottery_state.ticket_price != 0 {
        //     return Err(LotteryError::InvalidDeposit.into());
        // }

        // let user_balance = ctx.accounts.user.lamports();
        // if user_balance < deposit {
        //     return Err(LotteryError::InsufficientFunds.into());
        // }

        // let ticket_count = deposit
        //     .checked_div(lottery_state.ticket_price)
        //     .ok_or(LotteryError::Overflow)?;
        // if lottery_state
        //     .ticket_count
        //     .checked_add(ticket_count)
        //     .ok_or(LotteryError::Overflow)?
        //     > lottery_state.max_tickets
        // {
        //     return Err(LotteryError::InsufficientRemainingTickets.into());
        // }

        // ticket_data.ticket_count = ticket_data
        //     .ticket_count
        //     .checked_add(ticket_count)
        //     .ok_or(LotteryError::Overflow)?;

        // lottery_state.ticket_count = lottery_state
        //     .ticket_count
        //     .checked_add(ticket_count)
        //     .ok_or(LotteryError::Overflow)?;
        // lottery_state.prize_pool = lottery_state
        //     .prize_pool
        //     .checked_add(deposit)
        //     .ok_or(LotteryError::Overflow)?;

        // ticket_data.total_deposited = ticket_data
        //     .total_deposited
        //     .checked_add(deposit)
        //     .ok_or(LotteryError::Overflow)?;

        // Transfer lamports to the lottery vault
        // let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
        //     ctx.accounts.user.key,
        //     &ctx.accounts.lottery_vault.key(),
        //     deposit,
        // );

        // let accounts = [
        //     ctx.accounts.user.to_account_info(),
        //     ctx.accounts.lottery_vault.to_account_info(),
        //     ctx.accounts.system_program.to_account_info(),
        // ];

        msg!("CTX USER: {:?}", ctx.accounts.user.to_account_info());
        msg!(
            "CTX VAULT: {:?}",
            ctx.accounts.lottery_vault.to_account_info()
        );
        msg!(
            "CTX SYS: {:?}",
            ctx.accounts.system_program.to_account_info()
        );

        // anchor_lang::solana_program::program::invoke_signed(
        //     &transfer_instruction,
        //     &accounts,
        //     &[&[
        //         b"user_ticket_data",
        //         ctx.accounts.lottery_state.lottery_id.to_le_bytes().as_ref(),
        //         ctx.accounts.user.key.as_ref(),
        //     ]],
        // )?;

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
}
