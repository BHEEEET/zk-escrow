use anchor_lang::prelude::*;

declare_id!("7zr6gKJdnQQgA1mbpQXdu5KFabH2wfGqQEg88Dk1MuEs");

#[program]
pub mod zk_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
