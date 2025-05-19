use anchor_lang::prelude::*;
// use light_compressed_account::instruction_data::compressed_proof::CompressedProof;
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey ;
use light_system_program::invoke::processor::CompressedProof;
use light_compressed_token::{
    process_transfer::{
        CompressedTokenInstructionDataTransfer, InputTokenDataWithContext, PackedTokenTransferOutputData
    },
    program::LightCompressedToken,
};
use light_sdk::{cpi::invoke_light_system_program, light_system_accounts, LightTraits};

#[light_system_accounts]
#[derive(Accounts, LightTraits)]
pub struct MakeCompressedTokens<'info>{
    #[account(mut)]
    #[fee_payer]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub signer_account_info: AccountInfo<'info>,

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
    pub fn init_escrow(&mut self, amount: u64, input_token_data_with_context: Vec<InputTokenDataWithContext>, output_state_merkle_tree_account_indices: Vec<u8>, proof: CompressedProof) -> Result<()>{
        let escrow_token_data = PackedTokenTransferOutputData{
            amount,
            owner: solana_program::pubkey::Pubkey::from(self.escrow.key().to_bytes()),
            lamports: None,
            merkle_tree_index,
            tlv: None,
        };

        let change_token_data = create_change_output_compressed_token_account(
            &input_token_data_with_context,
            &[escrow_token_data.clone()],
            &self.signer.key(),
            output_state_merkle_tree_account_indices[1],
        );

        let output_compressed_accounts = vec![escrow_token_data, change_token_data];

        cpi_compressed_token_transfer(
            proof,
        )
        Ok(())
    }
    pub fn cpi_compressed_token_transfer (
        &mut self, 
        proof:CompressedProof,
        mint: Pubkey,
        _signer_is_delegate: bool,
        input_token_data_with_context: Vec<InputTokenDataWithContext>,
        output_compressed_accounts: Vec<PackedTokenTransferOutputData>
) -> Result<()>{
    let instruction = CompressedTokenInstructionDataTransfer{
        proof: Some(proof),
        mint,
        delegated_transfer: todo!(),
        input_token_data_with_context,
        output_compressed_accounts,
        is_compress: todo!(),
        compress_or_decompress_amount: todo!(),
        cpi_context: todo!(),
        lamports_change_account_merkle_tree_index: todo!(),
    }

    let account_info = 

        let cpi_accounts = invoke_light_system_program(&self.compressed_token_program.key, account_infos, instruction)

        Ok(())
    }
}


fn create_change_output_compressed_token_account(
    input_token_data_with_context: &[InputTokenDataWithContext],
    output_compressed_accounts: &[PackedTokenTransferOutputData],
    owner: &Pubkey,
    merkle_tree_index: u8,
) -> PackedTokenTransferOutputData {
    let input_sum = input_token_data_with_context
        .iter()
        .map(|account| account.amount)
        .sum::<u64>();
    let output_sum = output_compressed_accounts
        .iter()
        .map(|account| account.amount)
        .sum::<u64>();
    let change_amount = input_sum - output_sum;
    PackedTokenTransferOutputData {
        amount: change_amount,
        owner: solana_program::pubkey::Pubkey::from(owner.to_bytes()),
        lamports: None,
        merkle_tree_index,
        tlv: None,
    }
}