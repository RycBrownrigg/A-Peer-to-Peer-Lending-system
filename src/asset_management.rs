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
pub enum Asset {
    Digital(DigitalAsset),
    Physical(PhysicalAsset),
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DigitalAsset {
    pub token_address: Pubkey,
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PhysicalAsset {
    pub nft_address: Pubkey,
    pub metadata_uri: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum AssetInstruction {
    CreateDigitalAsset { token_address: Pubkey, amount: u64 },
    CreatePhysicalAsset { nft_address: Pubkey, metadata_uri: String },
    UpdateAssetAmount { new_amount: u64 },
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = AssetInstruction::try_from_slice(instruction_data)?;

    match instruction {
        AssetInstruction::CreateDigitalAsset { token_address, amount } => {
            process_create_digital_asset(accounts, token_address, amount, program_id)
        }
        AssetInstruction::CreatePhysicalAsset { nft_address, metadata_uri } => {
            process_create_physical_asset(accounts, nft_address, metadata_uri, program_id)
        }
        AssetInstruction::UpdateAssetAmount { new_amount } => {
            process_update_asset_amount(accounts, new_amount)
        }
    }
}

fn process_create_digital_asset(
    accounts: &[AccountInfo],
    token_address: Pubkey,
    amount: u64,
    program_id: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let asset_account = next_account_info(account_info_iter)?;

    if !asset_account.is_writable || asset_account.owner != program_id {
        return Err(ProgramError::InvalidAccountData.into());
    }

    let asset = Asset::Digital(DigitalAsset {
        token_address,
        amount,
    });

    asset.serialize(&mut *asset_account.data.borrow_mut())?;
    Ok(())
}

fn process_create_physical_asset(
    accounts: &[AccountInfo],
    nft_address: Pubkey,
    metadata_uri: String,
    program_id: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let asset_account = next_account_info(account_info_iter)?;

    if !asset_account.is_writable || asset_account.owner != program_id {
        return Err(ProgramError::InvalidAccountData.into());
    }

    let asset = Asset::Physical(PhysicalAsset {
        nft_address,
        metadata_uri,
    });

    asset.serialize(&mut *asset_account.data.borrow_mut())?;
    Ok(())
}

fn process_update_asset_amount(
    accounts: &[AccountInfo],
    new_amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let asset_account = next_account_info(account_info_iter)?;

    let mut asset = Asset::try_from_slice(&asset_account.data.borrow())?;
    
    match asset {
        Asset::Digital(ref mut digital_asset) => {
            digital_asset.amount = new_amount;
        }
        Asset::Physical(_) => {
            return Err(ProgramError::InvalidAccountData.into());
        }
    }

    asset.serialize(&mut *asset_account.data.borrow_mut())?;
    Ok(())
}