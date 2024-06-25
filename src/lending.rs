use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
    clock::Clock,
    sysvar::Sysvar,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum LendingInstruction {
    InitializeLendingPool { pool_seed: u64 },
    Deposit { amount: u64 },
    Borrow { amount: u64, collateral_amount: u64 },
    Repay { amount: u64 },
    Liquidate { loan_id: u64 },
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct LendingPool {
    pub total_deposits: u64,
    pub total_borrows: u64,
    pub interest_rate: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Loan {
    pub borrower: Pubkey,
    pub amount: u64,
    pub collateral: u64,
    pub start_time: i64,
    pub duration: i64,
    pub interest_rate: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = LendingInstruction::try_from_slice(instruction_data)?;

    match instruction {
        LendingInstruction::InitializeLendingPool { pool_seed } => {
            process_initialize_lending_pool(accounts, pool_seed, program_id)
        }
        LendingInstruction::Deposit { amount } => {
            process_deposit(accounts, amount)
        }
        LendingInstruction::Borrow { amount, collateral_amount } => {
            process_borrow(accounts, amount, collateral_amount)
        }
        LendingInstruction::Repay { amount } => {
            process_repay(accounts, amount)
        }
        LendingInstruction::Liquidate { loan_id } => {
            process_liquidate(accounts, loan_id)
        }
    }
}

fn process_initialize_lending_pool(
    accounts: &[AccountInfo],
    pool_seed: u64,
    program_id: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let lending_pool_account = next_account_info(account_info_iter)?;
    
    if !lending_pool_account.is_writable {
        return Err(ProgramError::InvalidAccountData.into());
    }

    let (expected_pubkey, _) = Pubkey::find_program_address(
        &[b"lending_pool", &pool_seed.to_le_bytes()],
        program_id,
    );

    if expected_pubkey != *lending_pool_account.key {
        return Err(ProgramError::InvalidAccountData.into());
    }

    let lending_pool = LendingPool {
        total_deposits: 0,
        total_borrows: 0,
        interest_rate: 500, // 5% as integer
    };

    lending_pool.serialize(&mut *lending_pool_account.data.borrow_mut())?;

    Ok(())
}

fn process_deposit(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user_account = next_account_info(account_info_iter)?;
    let lending_pool_account = next_account_info(account_info_iter)?;

    let mut lending_pool = LendingPool::try_from_slice(&lending_pool_account.data.borrow())?;
    lending_pool.total_deposits += amount;
    lending_pool.serialize(&mut *lending_pool_account.data.borrow_mut())?;

    // add - transfer tokens, update user account, etc.

    Ok(())
}

fn process_borrow(accounts: &[AccountInfo], amount: u64, collateral_amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user_account = next_account_info(account_info_iter)?;
    let lending_pool_account = next_account_info(account_info_iter)?;

    let mut lending_pool = LendingPool::try_from_slice(&lending_pool_account.data.borrow())?;
    lending_pool.total_borrows += amount;
    lending_pool.serialize(&mut *lending_pool_account.data.borrow_mut())?;

    let loan = Loan {
        borrower: *user_account.key,
        amount,
        collateral: collateral_amount,
        start_time: Clock::get()?.unix_timestamp,
        duration: 2592000, // 30 days in seconds
        interest_rate: lending_pool.interest_rate,
    };

    // add later - store the loan, transfer tokens, etc.

    Ok(())
}

fn process_repay(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user_account = next_account_info(account_info_iter)?;
    let lending_pool_account = next_account_info(account_info_iter)?;

    let mut lending_pool = LendingPool::try_from_slice(&lending_pool_account.data.borrow())?;
    lending_pool.total_borrows -= amount;
    lending_pool.serialize(&mut *lending_pool_account.data.borrow_mut())?;

    // add later - update the loan, transfer tokens, etc.

    Ok(())
}

fn process_liquidate(accounts: &[AccountInfo], loan_id: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let liquidator_account = next_account_info(account_info_iter)?;
    let borrower_account = next_account_info(account_info_iter)?;
    let lending_pool_account = next_account_info(account_info_iter)?;

    // add later - the check if the loan is eligible for liquidation,
    // transfer collateral to the liquidator, update the lending pool state, etc.

    Ok(())
}