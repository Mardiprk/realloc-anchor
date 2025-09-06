use anchor_lang::prelude::*;

declare_id!("F8hAoNSep1xzz3JRbwWrpN6p1wXVNGbMq6EiJQpXnoUS");

#[program]
pub mod note {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, text: String) -> Result<()> {
        let note = &mut ctx.accounts.note_account;
        note.owner = ctx.accounts.user.key();
        note.content = text;
        msg!(
            "{} Content has been stored on chain: {}",
            note.owner,
            note.content
        );
        Ok(())
    }
    pub fn append(ctx: Context<Append>, new_text: String) -> Result<()> {
        let note = &mut ctx.accounts.note_account;
        note.content.push_str(&new_text);
        msg!("New Content: {}", note.content);
        Ok(())
    }
}

#[account]
pub struct NoteAccount {
    pub owner: Pubkey,
    pub content: String,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 4 + 50,
        seeds = [b"note", user.key().as_ref()],
        bump
    )]
    pub note_account: Account<'info, NoteAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Append<'info> {
    #[account(
        mut,
        realloc = 8 + 32 + 4 + 200,
        realloc::payer = user,
        realloc::zero = false
    )]
    pub note_account: Account<'info, NoteAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
