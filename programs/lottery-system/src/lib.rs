use anchor_lang::prelude::*;


declare_id!("2dn3WTwz2MxZirrAiMaVGXggFV6gz8HP424N4bcaxP4X");

#[program]
pub mod lottery_system {
    use super::*;

    pub fn initialize_config(ctx: Context<Initialize>, start: u64, end: u64, ticket_price: u64) -> Result<()> {
       ctx.accounts.lottery_system.bump = ctx.bumps.lottery_system;
       ctx.accounts.lottery_system.start_time = start;
       ctx.accounts.lottery_system.end_time = end;
       ctx.accounts.lottery_system.ticket_price = ticket_price;
       ctx.accounts.lottery_system.authority = *ctx.accounts.payer.key;
       ctx.accounts.lottery_system.total_pot_amout = 0;
       ctx.accounts.lottery_system.total_ticket = 0;
       ctx.accounts.lottery_system.randomness_account = Pubkey::default();
       ctx.accounts.lottery_system.winner_claimed = false;
 
       Ok(())

    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + LotterySystem::INIT_SPACE,
        seeds = [b"lottery_system".as_ref()],
        bump

    )]
    pub lottery_system : Account<'info, LotterySystem>,

    pub system_program : Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct LotterySystem {
    pub bump: u8,
    pub winner: u64,
    pub winner_claimed: bool,
    pub start_time: u64,
    pub end_time: u64,
    pub lottery_amount : u64,
    pub ticket_price: u64,
    pub total_ticket: u64,
    pub total_pot_amout: u64,
    pub authority: Pubkey,
    pub randomness_account: Pubkey
}
