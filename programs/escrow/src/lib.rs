use anchor_lang::prelude::*;


declare_id!("F6adLHPL8g8pv3djDwC8J2UQ1xBDXmtAPB4SmLUwq8ZY");

pub mod state;
pub mod contexts;

use contexts::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, amount_a: u64, deposit: u64) -> Result<()> {
        ctx.accounts.init(seed, amount_a, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }


    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw()?;
        Ok(())
    }

    
}

#[derive(Accounts)]
pub struct Initialize {}
