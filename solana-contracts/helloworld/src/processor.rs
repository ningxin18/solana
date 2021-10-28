//! Program state processor

use crate::{
    instruction::{HelloWorldInstruction},
    state::{HelloWorldState},
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
    entrypoint::ProgramResult,
    msg,
    program_pack::{Pack},
    pubkey::Pubkey,
};

/// Program state handler.
pub struct Processor {}
impl Processor {

    /// Processes an [Instruction](enum.Instruction.html).
    pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = HelloWorldInstruction::unpack(input)?;

        match instruction {
            HelloWorldInstruction::Hello {
                message,
            } => {
                msg!("hello-world: HelloWorld");
                Self::process_hello(accounts, message)
            }
            HelloWorldInstruction::Erase=>{
                msg!("hello-world: Erase");
                Self::process_erase(accounts)
            }
        }
    }

    /// Processes an [Hello](enum.HelloWorldInstruction.html) instruction.
    fn process_hello(
        accounts: &[AccountInfo],
        message: String,
    ) -> ProgramResult {
        // 将消息内容和谁发的信息，进行记录
        let account_info_iter = &mut accounts.iter();
        let client_info = next_account_info(account_info_iter)?;
        let message_info = next_account_info(account_info_iter)?;

        // check permission
        if !client_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        msg!("before unpack hello");
        let mut state = HelloWorldState::unpack_unchecked(&message_info.data.borrow())?;
        msg!("after unpack hello");

        state.account_key = *client_info.key;
        state.message = message;

        msg!("before pack hello");
        HelloWorldState::pack(state, &mut message_info.data.borrow_mut())?;
        msg!("after pack hello");
        Ok(())
    }

    /// Processes a [Erase](enum.HelloWorldInstruction.html) instruction.
    pub fn process_erase(accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        // 用户传递instruction里面的keys数组就对应这里的accounts数组，用户将其创建的消息账号通过这个数组传递过来，通过next_account_info进行获取
        let client_info = next_account_info(account_info_iter)?;
        let message_info = next_account_info(account_info_iter)?;

        //check permission 用户构建transaction是否用了自己的进行签名，如果是的话，runtime会进行校验，因此只要判断他是否是被校验的单元,无需自己去调用鉴权接口。
        if !client_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let client_starting_lamports = client_info.lamports();
        **client_info.lamports.borrow_mut() = client_starting_lamports + message_info.lamports();
        **message_info.lamports.borrow_mut() = 0;
        Ok(())
    }
}