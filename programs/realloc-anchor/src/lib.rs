use anchor_lang::prelude::*;

declare_id!("F8hAoNSep1xzz3JRbwWrpN6p1wXVNGbMq6EiJQpXnoUS");

#[program]
pub mod realloc_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
