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
    pub fn pack(pool: &LiquidityPool, dst: &mut [u8]) -> Result<(), ProgramError> {
        if dst.len() < 49 {
            return Err(ProgramError::InvalidArgument);
        }

        dst[..32].copy_from_slice(&pool.authority.to_bytes());

        dst[32..40].copy_from_slice(&pool.total_liquidity.to_le_bytes());

        dst[40..48].copy_from_slice(&pool.available_liquidity.to_le_bytes());

        dst[48] = 1;

        Ok(())
    }

    pub fn unpack(src: &[u8]) -> Result<LiquidityPool, ProgramError> {
        if src.len() < 49 {
            // Si les données sont trop courtes, cela signifie que la pool n'existe pas
            // Créez une nouvelle instance de LiquidityPool avec les valeurs par défaut
            return Ok(LiquidityPool {
                authority: Pubkey::default(), // Utilisez la clé par défaut
                total_liquidity: 0,            // Aucune liquidité totale
                available_liquidity: 0,        // Aucune liquidité disponible
                is_initialized: false,         // La pool n'est pas initialisée
            });
        }
        let authority_bytes: [u8; 32] = src[..32].try_into().map_err(|_| ProgramError::InvalidAccountData)?;
        let authority = Pubkey::from(authority_bytes);

        let total_liquidity_bytes: [u8; 8] = src[32..40].try_into().map_err(|_| ProgramError::InvalidAccountData)?;
        let total_liquidity = u64::from_le_bytes(total_liquidity_bytes);

        let available_liquidity_bytes: [u8; 8] = src[40..48].try_into().map_err(|_| ProgramError::InvalidAccountData)?;
        let available_liquidity = u64::from_le_bytes(available_liquidity_bytes);

        let is_initialized = src[48] != 0;

        Ok(LiquidityPool {
            authority,
            total_liquidity,
            available_liquidity,
            is_initialized,
        })
    }
}

pub fn deposit_to_liquidity_pool(
    accounts: &[AccountInfo],
    _amount: u64,
) -> ProgramResult {
    // Vérifier les comptes
    let account_iter = &mut accounts.iter();
    let liquidity_pool_account = next_account_info(account_iter)?;
    let user_account = next_account_info(account_iter)?;

    let mut liquidity_pool_data_refcell = liquidity_pool_account.data.borrow_mut();
    let liquidity_pool_data_slice = &mut *liquidity_pool_data_refcell;
    let mut liquidity_pool_data = LiquidityPool::unpack(&liquidity_pool_data_slice)?;
    liquidity_pool_data.total_liquidity += 11;
    liquidity_pool_data.available_liquidity += 11;
    LiquidityPool::pack(&liquidity_pool_data, liquidity_pool_data_slice)?;

    transfer(user_account.key, liquidity_pool_account.key, 11);

    Ok(())
}

pub fn withdraw_from_liquidity_pool(
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    // Vérifier les comptes
    let account_iter = &mut accounts.iter();
    let liquidity_pool_account = next_account_info(account_iter)?;
    let user_account = next_account_info(account_iter)?;

    // Vérifier que le programme est le propriétaire de la pool de liquidité

    let liquidity_pool_data_refcell = liquidity_pool_account.data.borrow();
    let liquidity_pool_data_slice = &*liquidity_pool_data_refcell;
    let mut liquidity_pool_data = LiquidityPool::unpack(&liquidity_pool_data_slice)?;
    liquidity_pool_data.total_liquidity -= amount;
    liquidity_pool_data.available_liquidity -= amount;
    LiquidityPool::pack(&liquidity_pool_data, &mut liquidity_pool_account.data.borrow_mut())?;

    transfer(liquidity_pool_account.key, user_account.key, amount);

    Ok(())
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Vérifie l'instruction et détermine quelle fonction appeler en fonction de l'instruction_data
    if instruction_data == b"deposit" {
        // Appel de la fonction deposit_to_liquidity_pool avec un montant spécifique (par exemple, 10)
        deposit_to_liquidity_pool(accounts, 10)?;
    } 
    
    if instruction_data == b"withdraw" {
        withdraw_from_liquidity_pool(accounts, 10)?;
    }
    return Err(ProgramError::InvalidInstructionData);
}
