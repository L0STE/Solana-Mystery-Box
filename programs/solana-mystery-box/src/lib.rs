mod account;
mod state;
mod error;

use crate::account::*;
use crate::error::*;
pub use crate::state::*;

use anchor_lang::prelude::*;
use anchor_lang::system_program;
use solana_program::program::{invoke_signed, invoke};
use anchor_spl::token;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_mystery_box {
    use super::*;

    //Right now it's made so you can only use 1 token: Sol / Usdc / Whatever
    pub fn initizialize_box(
        ctx: Context<InitializeBox>,
        odd1: f32,
        odd2: f32,
        odd3: f32,
        odd4: f32,
        amount1: u64,
        amount2: u64,
        amount3: u64,
        amount4: u64,
        boxname: String,
        box_bump: u8,
    ) -> Result<()> {
        let box_state = &mut ctx.accounts.box_state;

        box_state.owner = *ctx.accounts.owner.key;
        box_state.boxname = boxname;
        box_state.odd1 = odd1;
        box_state.amount1 = amount1;
        box_state.odd2 = odd2;
        box_state.amount2 = amount2;
        box_state.odd3 = odd3;
        box_state.amount3 = amount3;
        box_state.odd4 = odd4;
        box_state.amount4 = amount4;
        box_state.rolled_times = 0;
        box_state.bank = 0;
        box_state.box_bump = box_bump;

        Ok(())
    }

    pub fn box_deposit(
        ctx: Context<BoxDeposit>,
        amount: u64,
    ) -> Result<()> {
        let box_state = &mut ctx.accounts.box_state;

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = token::Transfer{
            from: ctx.accounts.owner_ata.to_account_info(), 
            to: ctx.accounts.box_ata.to_account_info(),
            authority: ctx.accounts.owner.to_account_info()
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_context, amount)?; 

        box_state.bank += amount;

        Ok(())
    }

    pub fn box_withdraw(
        ctx: Context<BoxWithdraw>,
    ) -> Result<()> {
        let seeds = &[
            "box".as_bytes(),
            &ctx.accounts.box_state.key().clone().to_bytes(),
            &[ctx.accounts.box_state.box_bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = token::Transfer{
            from: ctx.accounts.box_ata.to_account_info(), 
            to: ctx.accounts.owner_ata.to_account_info(),
            authority: ctx.accounts.box_vault.to_account_info()
        };
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        
        
        token::transfer(cpi_context, ctx.accounts.box_state.bank)?; 

        ctx.accounts.box_state.bank = 0;

        Ok(())
    }

/* 
    //Right now it's FREE to roll the box, but in the future it will cost some SOL
    pub fn open_box(
        ctx: Context<OpenBox>,
    ) -> Result<()> {
        let box_state = &mut ctx.accounts.box_state;
        let box_invoice_state = &mut ctx.accounts.box_invoice_state;

        //MYSTERY BOX ROLLING ALGORITHM

        //Stage 1: Find the odd with more decimal number and how many decimal numbers it has
        let odds = [box_state.odd1, box_state.odd2, box_state.odd3, box_state.odd4];

        let mut max_decimal = 0;
        for &odd in &odds {
            if odd.fract() != 0.0 {
                let decimal = format!("{:?}", odd)
                .chars()
                .rev()
                .take_while(|ch| ch != &'.')
                .count() -1;
                if decimal > max_decimal {
                    max_decimal = decimal;
                }
            }
        }

        //Stage 2: Multiply by 10^n all the odd
        let odd1 = box_state.odd1 * 10_f32.powi(max_decimal as i32);
        let odd2 = box_state.odd2 * 10_f32.powi(max_decimal as i32);
        let odd3 = box_state.odd3 * 10_f32.powi(max_decimal as i32);
        let odd4 = box_state.odd4 * 10_f32.powi(max_decimal as i32);

        //Stage 3: Sums up all the new odd and create an Array of size sum_odd containing the 1,2,3,4 as weighted values
        let sum_odd = odd1 as u32 + odd2 as u32 + odd3 as u32 + odd4 as u32;

        let mut weighted_array = Vec::with_capacity(sum_odd as usize);

        for _ in 0..odd1 as u32 {
            weighted_array.push(1);
        }
        for _ in 0..odd2 as u32 {
            weighted_array.push(2);
        }
        for _ in 0..odd3 as u32 {
            weighted_array.push(3);
        }
        for _ in 0..odd4 as u32 {
            weighted_array.push(4);
        }

        //Stage 4: Fisher-Yates shuffle the array
        let mut rng = rand::thread_rng();
        let n = weighted_array.len();
        for i in (0..n).rev() {
            let j = rng.gen_range(0..=i);
            weighted_array.swap(i, j);
        }

        //Stage 5: Pick a number from 0 to sum_odd and reward the prize
        let winning_number = rng.gen_range(0..sum_odd);

        if weighted_array[winning_number] == 1 {
            let winning_odd = box_state.odd1;
            let winning_amount = box_state.amount1;
        } else if weighted_array[winning_number] == 2 {
            let winning_odd = box_state.odd2;
            let winning_amount = box_state.amount2;
        } else if weighted_array[winning_number] == 3 {
            let winning_odd = box_state.odd3;
            let winning_amount = box_state.amount3;
        } else if weighted_array[winning_number] == 4 {
            let winning_odd = box_state.odd4;
            let winning_amount = box_state.amount4;
        }

        let seeds = &[
            "box".as_bytes(),
            &ctx.accounts.box_account.key().clone().to_bytes(),
            &[box_state.box_bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = token::Transfer{
            from: ctx.accounts.box_ata.to_account_info(), 
            to: ctx.accounts.roller_ata.to_account_info(),
            authority: ctx.accounts.box_vault.to_account_info()
        };
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        
        token::transfer(cpi_context, winning_amount)?; 

        //BoxInvoice State Update
        box_invoice_state.roll = box_state.rolled_times;
        box_invoice_state.roller = *ctx.accounts.roller.key;
        box_invoice_state.box_id = *ctx.accounts.box_state.key;
        box_invoice_state.odd_won = winning_odd;
        box_invoice_state.amount_won = winning_amount;

        //Box State Update
        box_state.rolled_times += 1;
        box_state.bank -= winning_amount;

        Ok(())
    }

*/


}


