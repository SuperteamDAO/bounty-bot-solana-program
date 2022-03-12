use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use anchor_lang::solana_program::system_instruction;

declare_id!("EtxmAK5Hio4p1NQEwrRJu6UJpheKpBf1cfg9U7x3gGEX");

#[program]
pub mod escrow_mod {
    use super::*;

    pub fn lock_sol(ctx: Context<LockSOL>, amount: u64) -> Result<()> {
        let sender = &ctx.accounts.bounty_account.clone();
        let bounty_account = &mut ctx.accounts.bounty_account;
        bounty_account.is_claimed = false;
        bounty_account.bump = *ctx.bumps.get("bounty_account").unwrap();
        bounty_account.amount = amount;
        bounty_account.is_active = true;
        let ix = system_instruction::transfer(
            ctx.accounts.authority.key,
            &sender.to_account_info().key(),
            amount,
        );

        invoke(
            &ix,
            &[
                ctx.accounts.authority.to_account_info(),
                sender.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        Ok(())
    }

    pub fn claim_bounty(ctx: Context<ClaimBounty>) -> Result<()> {
        msg!("INF THE RPIOG");
        let ix = system_instruction::transfer(
            &ctx.accounts.bounty_account.to_account_info().key(),
            &ctx.accounts.reciever_account.to_account_info().key(),
            ctx.accounts.bounty_account.amount,
        );

        let seeds = &[
            b"bounty",
            ctx.accounts.authority.to_account_info().key.as_ref(),
            &[ctx.accounts.bounty_account.bump],
        ];
        let pda_signer = &[&seeds[..]];
        invoke_signed(
            &ix,
            &[
                ctx.accounts.bounty_account.to_account_info(),
                ctx.accounts.reciever_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            pda_signer,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LockSOL<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init,payer=authority,seeds=[b"bounty".as_ref(),authority.key().as_ref()],bump)]
    pub bounty_account: Account<'info, BountyAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimBounty<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,seeds=[b"bounty",authority.key().as_ref()],bump=bounty_account.bump)]
    pub bounty_account: Account<'info, BountyAccount>,
    #[account(mut)]
    pub reciever_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct BountyAccount {
    pub authority: Pubkey,
    pub amount: u64,
    pub is_claimed: bool,
    pub is_active: bool,
    pub bump: u8,
}
