use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

use spl_token_2022::ID as TOKEN_2022_PROGRAM_ID;
use spl_token_2022::{
    extension::StateWithExtensions,
    state::{Account, Mint},
};

use crate::{instruction::Batch0SaleInstruction, state::Batch0SaleProgramData};
pub struct Processor;
impl Processor {
    pub fn process(
        batch0_sale_program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = Batch0SaleInstruction::unpack(instruction_data)?;

        match instruction {
            Batch0SaleInstruction::InitSale { new_price_per_token } => {
                msg!("Instruction: init token sale program");
                Self::init_sale(accounts, new_price_per_token, batch0_sale_program_id)
            }
            Batch0SaleInstruction::UpdatePrice { new_price_per_token } => {
                msg!("Instruction: init token sale program");
                Self::update_price(accounts, new_price_per_token)
            }
            Batch0SaleInstruction::EndTokenSale {} => {
                msg!("Instruction : end token sale");
                Self::end_token_sale(accounts, batch0_sale_program_id)
            }
            Batch0SaleInstruction::BuyShot {} => {
                msg!("Instruction : buy a shot ");
                Self::buy_tokens(accounts, batch0_sale_program_id, 1000, 0)
            }
            Batch0SaleInstruction::BuyDouble {} => {
                msg!("Instruction : buy a shot ");
                Self::buy_tokens(accounts, batch0_sale_program_id, 2000, 100)
            }
            Batch0SaleInstruction::BuyFlask {} => {
                msg!("Instruction : buy a shot ");
                Self::buy_tokens(accounts, batch0_sale_program_id, 5000, 500)
            }
            Batch0SaleInstruction::BuyFifth {} => {
                msg!("Instruction : buy a shot ");
                Self::buy_tokens(accounts, batch0_sale_program_id, 25000, 5000)
            }
            Batch0SaleInstruction::BuyCase {} => {
                msg!("Instruction : buy a shot ");
                Self::buy_tokens(accounts, batch0_sale_program_id, 300000, 70000)
            }
            Batch0SaleInstruction::BuyBarrel {} => {
                msg!("Instruction : buy a shot ");
                Self::buy_tokens(accounts, batch0_sale_program_id, 1500000, 375000)
            }
        }
    }

    //inventory account info - Primary owner of the inventory
    //shelf ATA - Assignable ssociated Token Account holding the sellable inventory
    //till account info - receives tokens from sales
    //batch0 sale (token sale) program account info - Save the data about token sale
    //rent - To check if the rent fee is exempted
    //token 2022 program - To faciliate the change in ownership with the mint program
    fn init_sale(
        account_info_list: &[AccountInfo],
        new_price_per_token: u64,
        batch0_sale_program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut account_info_list.iter();

        if new_price_per_token == 0 {
            msg!("Price per token was 0");
            return Err(ProgramError::InvalidAccountData)
        }

        let inventory_account_info = next_account_info(account_info_iter)?;
        if !inventory_account_info.is_signer {
            msg!("Inventory Account must be a signer");
            return Err(ProgramError::MissingRequiredSignature);
        }

        msg!("Obtaining Till Account");
        let till_account_info = next_account_info(account_info_iter)?;

        msg!("Obtaining Shelf Account");
        let shelf_account_info = next_account_info(account_info_iter)?;

        msg!("Obtaining Prorgram Data Account");
        let batch0_sale_program_account_info = next_account_info(account_info_iter)?;

        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        if !rent.is_exempt(
            batch0_sale_program_account_info.lamports(),
            batch0_sale_program_account_info.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        //get data from account (needed `is_writable = true` option)
        msg!("Initializing Program Data");
        let mut batch0_sale_program_account_data = Batch0SaleProgramData::unpack_unchecked(
            &batch0_sale_program_account_info.try_borrow_data()?,
        )?;
        if batch0_sale_program_account_data.is_initialized {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        batch0_sale_program_account_data.init(
            true,
            new_price_per_token,
            *inventory_account_info.key,
            *shelf_account_info.key,
            *till_account_info.key,
        );

        Batch0SaleProgramData::pack(
            batch0_sale_program_account_data,
            &mut batch0_sale_program_account_info.try_borrow_mut_data()?,
        )?;

        let (pda, _bump_seed) =
            Pubkey::find_program_address(&[b"batch0_sale"], batch0_sale_program_id);

        msg!("Changing Authority for Shelf Account");
        let set_authority_ix = spl_token_2022::instruction::set_authority(
            &TOKEN_2022_PROGRAM_ID,
            shelf_account_info.key,
            Some(&pda),
            spl_token_2022::instruction::AuthorityType::AccountOwner,
            shelf_account_info.key,
            &[&shelf_account_info.key],
        )?;

        let token_2022_account_info = next_account_info(account_info_iter)?;
        // if *token_2022_account_info.key != spl_token_2022::id() {
        //     msg!("Error: Wrong token program account passed.");
        //     return Err(ProgramError::IncorrectProgramId);
        // }
        msg!("Change Shelf's Authority : Shelf Account -> Batch0 Program");
        invoke(
            &set_authority_ix,
            &[token_2022_account_info.clone(), shelf_account_info.clone()],
        )?;

        return Ok(());
    }

    fn update_price(
        account_info_list: &[AccountInfo],
        new_price_per_token: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut account_info_list.iter();

        if new_price_per_token == 0 {
            msg!("Price per token was 0");
            return Err(ProgramError::InvalidAccountData)
        }

        let inventory_account_info = next_account_info(account_info_iter)?;
        if !inventory_account_info.is_signer {
            msg!("Inventory Account must be a signer");
            return Err(ProgramError::MissingRequiredSignature);
        }

        msg!("Obtaining Prorgram Data Account");
        let batch0_sale_account_info = next_account_info(account_info_iter)?;
        let mut batch0_sale_account_data =
            Batch0SaleProgramData::unpack(&batch0_sale_account_info.try_borrow_data()?)?;

        if !batch0_sale_account_data.is_initialized {
            msg!("Program has not been initilaized");
            return Err(ProgramError::UninitializedAccount);
        }

        batch0_sale_account_data.current_price_per_token = new_price_per_token;

        Batch0SaleProgramData::pack(
            batch0_sale_account_data,
            &mut batch0_sale_account_info.try_borrow_mut_data()?,
        )?;

        return Ok(());
    }

    //buyer account info
    //till account info
    //temp token account info - For transfer the token to Buyer
    //token sale program account info - For getting data about TokenSaleProgram
    //system program - For transfer SOL
    //buyer token account info - For the buyer to receive the token
    //token program - For transfer the token
    //pda - For signing when send the token from temp token account
    // number_of_tokens - Amount of tokens user want to buy

    fn buy_tokens(
        accounts: &[AccountInfo],
        token_sale_program_id: &Pubkey,
        sold_tokens: u64,
        bonus_tokens: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let buyer_account_info = next_account_info(account_info_iter)?;
        if !buyer_account_info.is_signer {
            msg!("Buyer required to sign for purchase");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let batch0_sale_account_info = next_account_info(account_info_iter)?;
        let batch0_sale_account_data =
            Batch0SaleProgramData::unpack(&batch0_sale_account_info.try_borrow_data()?)?;

        if !batch0_sale_account_data.is_initialized {
            msg!("Program has not been initilaized");
            return Err(ProgramError::UninitializedAccount);
        }

        let till_account_info = next_account_info(account_info_iter)?;
        if *till_account_info.key != batch0_sale_account_data.till_pubkey {
            msg!("Invalid till account passed");
            return Err(ProgramError::InvalidAccountData);
        }

        let shelf_account_info = next_account_info(account_info_iter)?;
        if *shelf_account_info.key != batch0_sale_account_data.shelf_pubkey {
            msg!("Invalid shelf account passed");
            return Err(ProgramError::InvalidAccountData);
        }

        let purchase_lamports = sold_tokens * batch0_sale_account_data.current_price_per_token;
        msg!("Purchsing {} tokens for {} LAMPORTS", sold_tokens, purchase_lamports);

        msg!(
            "Transfer {} SOL : buy account -> seller account",
            purchase_lamports
        );
        let transfer_sol_to_seller = system_instruction::transfer(
            buyer_account_info.key,
            till_account_info.key,
            purchase_lamports,
        );

        let system_program = next_account_info(account_info_iter)?;
        invoke(
            &transfer_sol_to_seller,
            &[
                buyer_account_info.clone(),
                till_account_info.clone(),
                system_program.clone(),
            ],
        )?;

        msg!("transfer Token : shelf account -> buyer token account");
        let buyer_token_account_info = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;
        let token_mint_info = next_account_info(account_info_iter)?;
        let pda_account_info = next_account_info(account_info_iter)?;

        // fetch the number of decimals
        // let token_mint = spl_token_2022::state::Mint::unpack(&token_mint_info.data.borrow())?;
        // let decimals = token_mint.decimals;
        // Step 1: Borrow the account data and hold the reference
        let mint_data = token_mint_info.try_borrow_data()?;

        // Step 2: Unpack using StateWithExtensions
        let mint_with_extensions = StateWithExtensions::<Mint>::unpack(&mint_data)
            .map_err(|_| ProgramError::InvalidAccountData)?;

        let mint = mint_with_extensions.base;
        let decimals = mint.decimals;

        let (pda, bump_seed) =
            Pubkey::find_program_address(&[b"batch0_sale"], token_sale_program_id);

        let transfer_token_to_buyer_ix = spl_token_2022::instruction::transfer_checked(
            token_program.key,
            shelf_account_info.key,
            token_mint_info.key,
            buyer_token_account_info.key,
            &pda,
            &[&pda],
            sold_tokens + bonus_tokens,
            decimals,
        )
        .map_err(|_| ProgramError::InvalidInstructionData)?;

        invoke_signed(
            &transfer_token_to_buyer_ix,
            &[
                shelf_account_info.clone(),       // [writable] Source
                token_mint_info.clone(),          // [readable] Mint
                buyer_token_account_info.clone(), // [writable] Destination
                pda_account_info.clone(),  
                token_program.clone()
            ],
            &[&[&b"batch0_sale"[..], &[bump_seed]]],
        )?;

        return Ok(());
    }

    //inventory_token_account_info - To receive remainining token inventory
    //shelf_account_info - To send For retrieve remain token
    //token_2022_account_info - For transfer the token
    //pda - For signing when send the token from temp token account and close temp token account
    //batch0 sale program account info - To close token sale program
    fn end_token_sale(accounts: &[AccountInfo], batch0_sale_program_id: &Pubkey) -> ProgramResult {
        msg!("Ending the Sale");
        let account_info_iter = &mut accounts.iter();
        let program_data_account_info = next_account_info(account_info_iter)?;

        msg!("Attempting to get the state data");
        // get state data
        let batch0_sale_program_account_data =
            Batch0SaleProgramData::unpack_unchecked(&program_data_account_info.try_borrow_data()?)?;

        msg!(
            "Is Initialized {}",
            batch0_sale_program_account_data.is_initialized
        );
        if !batch0_sale_program_account_data.is_initialized {
            msg!("Program has not been initilaized");
            return Err(ProgramError::UninitializedAccount);
        }

        msg!("Verify inventory account passed");
        let inventory_account_info = next_account_info(account_info_iter)?;
        if *inventory_account_info.key != batch0_sale_program_account_data.inventory_pubkey {
            return Err(ProgramError::InvalidAccountData);
        }

        msg!("Verify shelf account passed");
        let shelf_account_info = next_account_info(account_info_iter)?;
        if *shelf_account_info.key != batch0_sale_program_account_data.shelf_pubkey {
            return Err(ProgramError::InvalidAccountData);
        }

        let (pda, bump) = Pubkey::find_program_address(&[b"batch0_sale"], batch0_sale_program_id);

        msg!("shelf_account_info.key: {}", shelf_account_info.key);
        msg!("shelf_account_info.owner: {}", shelf_account_info.owner);
        msg!("expected owner: {}", spl_token_2022::id());

        msg!("transfer Token : shelf account -> inventory account");
        let shelf_account_binding = shelf_account_info.data.borrow();
        let account_with_extensions =
            StateWithExtensions::<Account>::unpack(&shelf_account_binding)
                .map_err(|_| ProgramError::InvalidAccountData)?;

        let shelf_account_info_data = &account_with_extensions.base;

        msg!("getting mint data for transfer");
        let token_mint_info = next_account_info(account_info_iter)?;
        let mint_data = token_mint_info.try_borrow_data()?;
        let mint_with_extensions = StateWithExtensions::<Mint>::unpack(&mint_data)
            .map_err(|_| ProgramError::InvalidAccountData)?;

        let mint = mint_with_extensions.base;
        let decimals = mint.decimals;

        let inventory_ata_info = next_account_info(account_info_iter)?;
        let return_to_owner_ix = spl_token_2022::instruction::transfer_checked(
            &spl_token_2022::ID,
            shelf_account_info.key,
            token_mint_info.key,
            inventory_ata_info.key,
            &pda,
            &[&pda],
            shelf_account_info_data.amount,
            decimals,
        )
        .map_err(|_| ProgramError::InvalidInstructionData)?;

        drop(shelf_account_binding);
        let token_2022_account_info = next_account_info(account_info_iter)?;
        let pda_account_info = next_account_info(account_info_iter)?;
        invoke_signed(
            &return_to_owner_ix,
            &[
                token_2022_account_info.clone(),
                shelf_account_info.clone(),
                token_mint_info.clone(),
                inventory_ata_info.clone(),
                pda_account_info.clone(),
            ],
            &[&[&b"batch0_sale"[..], &[bump]]],
        )?;

        msg!("close token sale program");
        **inventory_account_info.try_borrow_mut_lamports()? = inventory_account_info
            .lamports()
            .checked_add(program_data_account_info.lamports())
            .ok_or(ProgramError::InsufficientFunds)?;
        **program_data_account_info.try_borrow_mut_lamports()? = 0;
        let mut data = program_data_account_info.try_borrow_mut_data()?;
        for byte in data.iter_mut() {
            *byte = 0;
        }

        return Ok(());
    }
}
