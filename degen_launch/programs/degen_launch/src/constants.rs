use anchor_lang::prelude::*;
use std::str::FromStr;

// Program and Token IDs
pub const SOLEND_PROGRAM_ID: &str = "LendZqTs7gn5CTSJU1jWKhKuVpjJGom45nnwPb2AMTi";
pub const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const RENT_SYSVAR_ID: &str = "SysvarRent111111111111111111111111111111111";

// Token Mints
pub const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
pub const DEGEN_MINT: &str = "E1YrvUKxBzrx5mGtL4D3VB12Jxr8xn1ibpZq6XCGX7es";

// Solend Specific Addresses
pub const SOLEND_USDC_RESERVE: &str = "BgxfHJDzm44T7XG68MYKx7YisTjZu73tVovyZSjJMpmw";
pub const RESERVE_LIQUIDITY_SUPPLY: &str = "8SheGtsopRUDzdiD6v6BR9a6bqZ9QwywYQY99Fp5meNf";
pub const LENDING_MARKET_AUTHORITY: &str = "DdZR6zRFiUt4S5mg7AV1uKB2z1f1WzcNYCaTEEWPAuby";

// User Accounts
pub const USER_WALLET: &str = "GHLkM2szK28hwq2uLYH361euMUUw441Ec3TZJf39owA6";
pub const USER_USDC_ACCOUNT: &str = "FXsHUq4au79ZLmA9FiXB44cDJKzxHgWuPnLij816kHPZ";
pub const USER_DEGEN_ACCOUNT: &str = "E6dvNzzQ8cztRPzhLo4mRy9hhSywxHZQx9kzqdzUTeY5";

pub fn get_program_id() -> Pubkey {
    Pubkey::from_str("6UBFGLf5YBdVAzdzzoMhQsL3pM1KjgRp7EgVDCP4UqGV").unwrap()
}

pub fn get_token_program_id() -> Pubkey {
    Pubkey::from_str(TOKEN_PROGRAM_ID).unwrap()
}

pub fn get_solend_program_id() -> Pubkey {
    Pubkey::from_str(SOLEND_PROGRAM_ID).unwrap()
}