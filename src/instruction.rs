use solana_program::{msg, program_error::ProgramError};
use std::convert::TryInto;

use crate::error::CustomError::InvalidInstruction;

pub enum Batch0SaleInstruction {
    InitSale { new_price_per_token: u64 },
    UpdatePrice { new_price_per_token: u64 },
    EndTokenSale {},
    BuyShot {},
    BuyDouble {},
    BuyFlask {},
    BuyFifth {},
    BuyCase {},
    BuyBarrel {},
}

//function of enum
impl Batch0SaleInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use solana_program::msg;
        msg!("Raw instruction data: {:?}", input);
        //check instruction type
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        //unpack the rest data for each instruction
        return match tag {
            0 => Ok(Self::InitSale {
                new_price_per_token: Self::unpack_byte(rest, 0)?,
            }),
            1 => Ok(Self::UpdatePrice {
                new_price_per_token: Self::unpack_byte(rest, 0)?,
            }),
            2 => Ok(Self::EndTokenSale {}),
            3 => Ok(Self::BuyShot {}),
            4 => Ok(Self::BuyDouble {}),
            5 => Ok(Self::BuyFlask {}),
            6 => Ok(Self::BuyFifth {}),
            7 => Ok(Self::BuyCase {}),
            8 => Ok(Self::BuyBarrel {}),
            _ => Err(InvalidInstruction.into()),
        };
    }
    fn unpack_byte(input: &[u8], byte_index: usize) -> Result<u64, ProgramError> {
        let start_bit = byte_index * 8;
        let end_bit = start_bit + 8;

        let data = input
            .get(start_bit..end_bit)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;

        return Ok(data);
    }
}
