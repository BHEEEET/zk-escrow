use anchor_lang::prelude::*;
mod instructions;
mod state;
use crate::instructions::*;

declare_id!("7zr6gKJdnQQgA1mbpQXdu5KFabH2wfGqQEg88Dk1MuEs");

#[program]
pub mod zk_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Make>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
