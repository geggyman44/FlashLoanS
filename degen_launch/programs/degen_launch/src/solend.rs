use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};

// Flash loan instruction discriminator for Solend
pub const FLASH_LOAN_IX: u8 = 12;

// Required by Solend's flash loan instruction
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct FlashLoanParams {
    pub amount: u64,
}

pub fn flash_loan_ix(
    program_id: Pubkey,
    source_liquidity: Pubkey,
    destination_liquidity: Pubkey,
    reserve: Pubkey,
    lending_market_authority: Pubkey,
    amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(program_id, false),
        AccountMeta::new(source_liquidity, false),
        AccountMeta::new(destination_liquidity, false),
        AccountMeta::new(reserve, false),
        AccountMeta::new_readonly(lending_market_authority, false),
    ];

    let data = FlashLoanParams { amount };
    let mut encoded = vec![FLASH_LOAN_IX];
    encoded.extend_from_slice(&data.try_to_vec().unwrap());

    // Use the constants here

    Instruction {
        program_id,
        accounts,
        data: encoded,
    }
}

// Constants for Solend's program
pub mod constants {
    use anchor_lang::prelude::*;
    
    // USDC reserve on mainnet
    pub static USDC_RESERVE: &str = "BgxfHJDzm44T7XG68MYKx7YisTjZu4NSrPCEZwNdfGH4";
}