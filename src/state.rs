use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct Batch0SaleProgramData {
    pub is_initialized: bool,
    pub current_price_per_token: u64,
    pub inventory_pubkey: Pubkey,
    pub shelf_pubkey: Pubkey,
    pub till_pubkey: Pubkey
}

impl Batch0SaleProgramData {
    pub fn init(
        &mut self,
        is_initialized: bool,              // 1
        current_price_per_token: u64, // 64
        inventory_pubkey: Pubkey,              // 32
        shelf_pubkey: Pubkey, // 32
        till_pubkey: Pubkey // 32
    ) {
        self.is_initialized = is_initialized;
        self.current_price_per_token = current_price_per_token;
        self.inventory_pubkey = inventory_pubkey;
        self.shelf_pubkey = shelf_pubkey;
        self.till_pubkey = till_pubkey;
    }
}

impl Sealed for Batch0SaleProgramData {}

impl IsInitialized for Batch0SaleProgramData {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Batch0SaleProgramData {
    const LEN: usize = 105; // 1 + 8 + 32 + 32 + 32
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Batch0SaleProgramData::LEN];
        let (
            is_initialized,
            current_price_bytes,
            inventory_pubkey,
            shelf_pubkey,
            till_pubkey,
        ) = array_refs![src, 1, 8, 32, 32, 32]; 

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let current_price_per_token = u64::from_le_bytes(*current_price_bytes);

        return Ok(Batch0SaleProgramData {
            is_initialized,
            current_price_per_token,
            inventory_pubkey: Pubkey::new_from_array(*inventory_pubkey),
            shelf_pubkey: Pubkey::new_from_array(*shelf_pubkey),
            till_pubkey: Pubkey::new_from_array(*till_pubkey),
        });
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Batch0SaleProgramData::LEN];
        let (
            is_initialized_dst,
            current_price_per_token_dst,
            inventory_pubkey_dst,
            shelf_pubkey_dst,
            till_pubkey_dst,
        ) = mut_array_refs![dst, 1, 8, 32, 32, 32];

        let Batch0SaleProgramData {
            is_initialized,
            current_price_per_token,
            inventory_pubkey,
            shelf_pubkey,
            till_pubkey
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        current_price_per_token_dst.copy_from_slice(&current_price_per_token.to_le_bytes());
        inventory_pubkey_dst.copy_from_slice(inventory_pubkey.as_ref());
        shelf_pubkey_dst.copy_from_slice(shelf_pubkey.as_ref());
        till_pubkey_dst.copy_from_slice(till_pubkey.as_ref());
    }
}
