pub struct Loan {
    pub borrower: Pubkey,
    pub amount_borrowed: u64,
    pub collateral_amount: u64, // what is collateral amount ? 
    pub is_active: bool,
}
