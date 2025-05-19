use anchor_lang::prelude::*;
use light_compressed_account::instruction_data::compressed_proof::CompressedProof;
use light_compressed_token::{
    process_transfer::{
        
    },
    program::LightCompressedAccount,
};
use light_sdk::{light_system_accounts, LightTraits};

#[light_system_accounts]
#[derive(Accounts, LightTraits)]
pub struct MakeCompressedTokens<'info>{
    #[account(mut)]
    #[fee_payer]
    pub signer: Signer<'info>,

    #[authority]
    #[account(
        seeds = [b"escrow".as_slice(), signer.key.to_bytes().as_slice(),
        bump
        ]
    )]
    pub escrow: AccountInfo<'info>,

    #[self_program]
    pub compressed_token_program: Program<'info, LightCompressedToken>,

    pub compressed_token_cpi_authority: AccountInfo<'info>,
}

impl<'info> MakeCompressedTokens<'info>{
    pub fn init_escrow(&mut self, amount: u64) -> Result<()>{
        Ok(())
    }
}