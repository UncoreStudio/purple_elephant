use std::io::Write;
use arrayref::array_ref;
use solana_program::{
    account_info::{next_account_info, AccountInfo}, 
    entrypoint::ProgramResult,
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey, system_instruction::transfer
};

pub struct LiquidityPool {
    pub authority: Pubkey,
    pub total_liquidity: u64,
    pub available_liquidity: u64,
    pub is_initialized: bool,
}

impl LiquidityPool {
    pub fn pack(pool: Self, dst: &mut [u8]) -> Result<(), ProgramError> {
        let mut writer = std::io::Cursor::new(dst);
        writer.write_all(&pool.authority.to_bytes())?;
        writer.write_all(&pool.total_liquidity.to_le_bytes())?;
        writer.write_all(&pool.available_liquidity.to_le_bytes())?;
        writer.write_all(&[if pool.is_initialized { 1 } else { 0 }])?;
        Ok(())
    }

    // Fonction pour désérialiser les octets en une structure de pool de liquidité
    pub fn unpack(src: &[u8]) -> Result<Self, ProgramError> {
        let _cursor = std::io::Cursor::new(src);
        let authority = Pubkey::new_from_array(*array_ref![src, 0, 32]);
        let total_liquidity = u64::from_le_bytes(*array_ref![src, 32, 8]);
        let available_liquidity = u64::from_le_bytes(*array_ref![src, 40, 8]);
        let is_initialized_byte = array_ref![src, 48, 1][0]; // Obtenez le premier octet à partir de l'index 48
        let is_initialized = is_initialized_byte != 0; // Convertissez l'octet en booléen
        Ok(Self {
            authority,
            total_liquidity,
            available_liquidity,
            is_initialized,
        })
    }
}

pub fn deposit_to_liquidity_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    // Vérifier les comptes
    let account_iter = &mut accounts.iter();
    let liquidity_pool_account = next_account_info(account_iter)?;
    let user_account = next_account_info(account_iter)?;

    // Vérifier que le programme est le propriétaire de la pool de liquidité
    if *liquidity_pool_account.owner != *program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut liquidity_pool_data = LiquidityPool::unpack(&liquidity_pool_account.data.borrow())?;
    liquidity_pool_data.total_liquidity += amount;
    liquidity_pool_data.available_liquidity += amount;
    LiquidityPool::pack(liquidity_pool_data, &mut liquidity_pool_account.data.borrow_mut())?;

    transfer(user_account.key, liquidity_pool_account.key, amount);

    Ok(())
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Vérifie l'instruction et détermine quelle fonction appeler en fonction de l'instruction_data
    if instruction_data == b"deposit" {
        // Appel de la fonction deposit_to_liquidity_pool avec un montant spécifique (par exemple, 1000)
        deposit_to_liquidity_pool(program_id, accounts, 1000)?;
    } else {
        // Gérer d'autres instructions
        return Err(ProgramError::InvalidInstructionData);
    }

    Ok(())
}
