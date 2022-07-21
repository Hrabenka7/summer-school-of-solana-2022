use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod turnstile {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.locked = true;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub state: Account<'info, State>,
    pub user: Signer<'info>,
    pub system_program: Program<'info,System>
}

#[account]
pub struct State {
    pub locked: bool,
}
