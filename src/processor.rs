use solana_program::{
    account_info::{
        AccountInfo,
        next_account_info,
    },
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
};
use std::convert::TryInto;

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let src_info = next_account_info(account_info_iter)?;
    let dst_info = next_account_info(account_info_iter)?;
    let lamports = instruction_data
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidArgument)?;
    **src_info.try_borrow_mut_lamports()? -= lamports;
    **dst_info.try_borrow_mut_lamports()? += lamports;
    Ok(())
}
