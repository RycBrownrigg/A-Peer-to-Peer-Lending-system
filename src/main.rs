/* 1.	System Architecture

I'll use Solana as the primary platform for the blockchain layer due to its high throughput and low transaction costs. I'll develop smart contracts in Rust using Solana's programming model. Here's the structure for the main lending program:

*/
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
};
use borsh::{BorshDeserialize, BorshSerialize};

mod lending;
mod user_management;
mod asset_management;

use lending::LendingInstruction;
use user_management::UserInstruction;
use asset_management::AssetInstruction;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum ProgramInstruction {
    Lending(LendingInstruction),
    User(UserInstruction),
    Asset(AssetInstruction),
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ProgramInstruction::try_from_slice(instruction_data)?;

    match instruction {
        ProgramInstruction::Lending(lending_instruction) => {
            lending::process_instruction(program_id, accounts, &lending_instruction.try_to_vec()?)
        }
        ProgramInstruction::User(user_instruction) => {
            user_management::process_instruction(program_id, accounts, &user_instruction.try_to_vec()?)
        }
        ProgramInstruction::Asset(asset_instruction) => {
            asset_management::process_instruction(program_id