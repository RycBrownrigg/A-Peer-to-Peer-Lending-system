use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct User {
    pub did: String,
    pub reputation_score: u32,
    pub kyc_status: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum UserInstruction {
    CreateUser { did: String },
    UpdateReputation { new_score: u32 },
    SetKycStatus { status: bool },
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = UserInstruction::try_from_slice(instruction_data)?;

    match instruction {
        UserInstruction::CreateUser { did } => {
            process_create_user(accounts, did, program_id)
        }
        UserInstruction::UpdateReputation { new_score } => {
            process_update_reputation(accounts, new_score)
        }
        UserInstruction::SetKycStatus { status } => {
            process_set_kyc_status(accounts, status)
        }
    }
}

fn process_create_user(
    accounts: &[AccountInfo],
    did: String,
    program_id: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user_account = next_account_info(account_info_iter)?;

    if !user_account.is_writable || user_account.owner != program_id {
        return Err(ProgramError::InvalidAccountData.into());
    }

    let user = User {
        did,
        reputation_score: 0,
        kyc_status: false,
    };

    user.serialize(&mut *user_account.data.borrow_mut())?;
    Ok(())
}

fn process_update_reputation(
    accounts: &[AccountInfo],
    new_score: u32,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user_account = next_account_info(account_info_iter)?;

    let mut user = User::try_from_slice(&user_account.data.borrow())?;
    user.reputation_score = new_score;
    user.serialize(&mut *user_account.data.borrow_mut())?;
    Ok(())
}

fn process_set_kyc_status(
    accounts: &[AccountInfo],
    status: bool,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user_account = next_account_info(account_info_iter)?;

    let mut user = User::try_from_slice(&user_account.data.borrow())?;
    user.kyc_status = status;
    user.serialize(&mut *user_account.data.borrow_mut())?;
    Ok(())
}