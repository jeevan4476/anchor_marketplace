use anchor_lang::prelude::*;

#[account]
// #[derive(InitSpace)]
pub struct Marketplace {
    pub admin: Pubkey,
    pub fee: u16,
    pub bump: u8,
    pub tresury_bump: u8,
    pub rewards_bump: u8,
    pub name: String,
}
impl Space for Marketplace {
    const INIT_SPACE: usize = 8 + 32 + 2 + 1 + 1 + 1 + (4 + 32);
}
