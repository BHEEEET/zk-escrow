use anchor_lang::prelude::*;
use light_compressed_account::instruction_data::compressed_proof::CompressedProof;
use light_compressed_token::{
    process_transfer::{
        CompressedTokenInstructionDataTransfer, InputTokenDataWithContext, PackedTokenTransferOutputData
    },
    program::LightCompressedToken,
};
use light_sdk::{light_system_accounts, LightTraits};
use crate::create_change_output_compressed_token_account;

#[light_system_accounts]
#[derive(Accounts, LightTraits)]
pub struct MakeCompressedTokens<'info>{
    #[account(mut)]
    #[fee_payer]
    pub signer: Signer<'info>,

    #[authority]
    #[account(
        seeds = [b"escrow".as_slice(), signer.key.to_bytes().as_slice()],
        bump
    )]
    pub escrow: AccountInfo<'info>,

    #[self_program]
    pub compressed_token_program: Program<'info, LightCompressedToken>,

    pub compressed_token_cpi_authority: AccountInfo<'info>,
}

impl<'info> MakeCompressedTokens<'info>{
    pub fn init_escrow(&mut self, amount: u64) -> Result<()>{
        let escrow_token_data = PackedTokenTransferOutputData{
            amount,
            owner: solana_program::pubkey::Pubkey::from(self.escrow.key().to_bytes()),
            lamports: None,
            merkle_tree_index,
            tlv: None,
        };

        let change_token_data = create_change
        Ok(())
    }
}