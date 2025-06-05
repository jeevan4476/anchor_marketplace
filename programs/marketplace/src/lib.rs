use anchor_lang::prelude::*;

mod state;
use state::*;

mod instructions;
use instructions::*;

declare_id!("8tbky7bfxdJLndoWf1RRsnPwnCZNz2QyDviYAjsX2vdT");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
