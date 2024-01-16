use borsh::{BorshDeserialize, BorshSerialize};
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::instructions::CounterInstructions;

pub mod instructions;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterAccount {
    pub counter: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {

    msg!("Initializing counter program");

    let instruction = CounterInstructions::unpack(instruction_data)?;
    let counter_iter = &mut accounts.iter();
    let account = next_account_info(counter_iter)?;
    
    let mut counter_account = CounterAccount::try_from_slice(&account.try_borrow_data()?)?;

    match instruction {
        CounterInstructions::Increment(args) => {
            counter_account.counter += args.value;
        },
        CounterInstructions::Decrement(args) => {
            if counter_account.counter > args.value {
                counter_account.counter -= args.value;
            } else {
                counter_account.counter = 0;
            }
        },
        CounterInstructions::Reset => {
            counter_account.counter = 0;
        },
        CounterInstructions::Update(args) => {
            counter_account.counter = args.value;
        }
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_counter() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default()
        );

        let accounts = vec![account];

        let increment_instruction_data = vec![0];
        let decrement_instruction_data = vec![1];
        let mut update_instruction_data = vec![2];
        let reset_instruction_data = vec![3];

        process_instruction(
            &program_id, 
            &accounts, 
            &increment_instruction_data
        ).unwrap();

        assert_eq!(CounterAccount::try_from_slice(&accounts[0].data.borrow()).unwrap().counter, 1);

        process_instruction(
            &program_id, 
            &accounts, 
            &decrement_instruction_data
        ).unwrap();

        assert_eq!(CounterAccount::try_from_slice(&accounts[0].data.borrow()).unwrap().counter, 0);

        let update_value = 33u32;
        update_instruction_data.extend_from_slice(&update_value.to_le_bytes());
        process_instruction(
            &program_id, 
            &accounts, 
            &update_instruction_data
        ).unwrap();

        assert_eq!(CounterAccount::try_from_slice(&accounts[0].data.borrow()).unwrap().counter, 33);

        process_instruction(
            &program_id, 
            &accounts, 
            &reset_instruction_data
        ).unwrap();

        assert_eq!(CounterAccount::try_from_slice(&accounts[0].data.borrow()).unwrap().counter, 0);

    }
}