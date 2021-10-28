//! State transition types

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
    msg,
};
use std::str::from_utf8;

///state用来将内容存储到对应的文件时，存储格式的定义，类似一个ORM或者所谓的MVC中Model层
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HelloWorldState {
    /// account
    pub account_key: Pubkey,
    /// message 
    pub message: String
}

impl Sealed for HelloWorldState {}

impl IsInitialized for HelloWorldState {
    fn is_initialized(&self) -> bool {
        return true;
    }
}

///SDK提供的Pack trate来实现其序列化和反序列化，
impl Pack for HelloWorldState {
    //当前Solana上，Account仅可以初始化一次长度信息，创建后不可更改。
    const LEN: usize = 32+1+256; // max hello message's length is 256

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        //通过array_refs指定三个成员的内容，这里我们在序列化文件内容时，采用和Instruction一样的二进制序列化方法，
        //对于Pubkey其固定为32个字节。
        //对于Message，其长度约定小于256，这样用一个字节表示长度，后面256个字节表示内容（256不一定全部用完，分配空间)
        let src = array_ref![src, 0, 289];
        let (account_key_buf, message_len_buf, message_buf) = array_refs![src, 32, 1, 256];
        let account_key = Pubkey::new_from_array(*account_key_buf);
        let message_len = message_len_buf[0] as u8;
        let (msg_buf, _rest) = message_buf.split_at(message_len.into());
        let message = String::from(from_utf8(msg_buf).unwrap()) ;
        Ok(HelloWorldState {
            account_key,
            message
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 289];
        //mut_array_refs预先给几个要存储的元素分配好地址，然后使用copy_from_slice复制32字节的key，用as u8转换长度，copy_from_slice copy字符串内容
        let (
            account_key_buf,
            message_len_buf,
            message_buf,
        ) = mut_array_refs![dst, 32, 1, 256];
        account_key_buf.copy_from_slice(self.account_key.as_ref());
        message_len_buf[0] = self.message.len() as u8;
        msg!("pack copy slice");
        message_buf[..self.message.len()].copy_from_slice(&self.message.as_bytes());
    }
}