use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::state::*;

#[derive(Accounts)]
pub struct InitializeBox<'info> {
    #[account(
        init,
        payer = owner,
        space = Box::space() + 20, //Max Length of String = 20
    )]
    pub box_account: Account<'info, Box>,
    #[account(seeds = [b"box", box_account.key().as_ref()], bump)]
    pub box_vault: SystemAccount<'info>,

    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BoxDeposit<'info> {
    #[account(
        mut,
        constraint = box_account.owner == *owner.key,
    )]
    pub box_account: Account<'info, Box>,
    #[account(seeds = [b"box", box_account.key().as_ref()], bump = box_account.box_bump)]
    pub box_vault: SystemAccount<'info>,
    #[account(mut)]
    pub box_ata: Account<'info, TokenAccount>,

    pub owner: Signer<'info>,
    pub owner_ata: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BoxWithdraw<'info> {
    #[account(
        mut,
        constraint = box_account.owner == *owner.key,
    )]
    pub box_account: Account<'info, Box>,
    #[account(seeds = [b"box", box_account.key().as_ref()], bump = box_account.box_bump)]
    pub box_vault: SystemAccount<'info>,
    #[account(mut)]
    pub box_ata: Account<'info, TokenAccount>,

    pub owner: Signer<'info>,
    pub owner_ata: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

//ToDo
#[derive(Accounts)]
pub struct OpenBox<'info> {
    #[account(
        mut,
        constraint = box_account.bank > box_account.amount1,
        constraint = box_account.bank > box_account.amount2,
        constraint = box_account.bank > box_account.amount3,
        constraint = box_account.bank > box_account.amount4,
    )]
    pub box_account: Account<'info, Box>,
    #[account(seeds = [b"box", box_account.key().as_ref()], bump = box_account.box_bump)]
    pub box_vault: SystemAccount<'info>,
    #[account(mut)]
    pub box_ata: Account<'info, TokenAccount>,

    pub roller: Signer<'info>,
    pub roller_ata: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}