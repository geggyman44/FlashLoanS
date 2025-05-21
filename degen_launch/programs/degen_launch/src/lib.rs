use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{AccountMeta, Instruction}, program::invoke};
use anchor_spl::token::{Token, TokenAccount, Mint};
use whirlpool_cpi::state::{WhirlpoolRewardInfo, Whirlpool, TickArray, FeeTier};
use crate::whirlpool_utils::WhirlpoolPdas;

pub mod solend;
pub mod swap_via_orca;
mod constants;
mod whirlpool_utils;

#[allow(unused_imports)]
use solend::flash_loan_ix;
#[allow(unused_imports)]
use swap_via_orca::SwapViaOrca;


declare_id!("6UBFGLf5YBdVAzdzzoMhQsL3pM1KjgRp7EgVDCP4UqGV");

#[derive(Accounts)]
pub struct TriggerFlashloan<'info> {
    /// SPL Token Lending program (Solend uses this under the hood)
    pub lending_program: Program<'info, FlashLoanProgram>,
    /// CHECK: This is validated by the lending program
    pub reserve: AccountInfo<'info>,
    /// CHECK: Not read by our program, just passed through
    pub reserve_liquidity_supply: AccountInfo<'info>,
    /// CHECK: Validated by Solend program
    pub lending_market_authority: AccountInfo<'info>,
    /// CHECK: This is the destination for borrowed funds
    #[account(mut)]
    pub user_liquidity: AccountInfo<'info>,
    /// Token program
    pub token_program: Program<'info, Token>,
}

#[program]
pub mod degen_launch {
    use super::*;

    // Add this new function alongside your existing execute_flashloan_selfdump
    pub fn start_flashloan(
        ctx: Context<TriggerFlashloan>,
        amount: u64,
        minimum_amount_out: u64,
        _is_buy: bool,
    ) -> Result<()> {
        // Build the callback instruction that will be executed after receiving the flash loan
        let accounts = vec![
            AccountMeta::new(ctx.accounts.reserve.key(), false),
            AccountMeta::new(ctx.accounts.user_liquidity.key(), false),
            AccountMeta::new(ctx.accounts.reserve_liquidity_supply.key(), false),
            AccountMeta::new(ctx.accounts.lending_market_authority.key(), false),
            AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
            // Add all other required accounts for execute_flashloan_selfdump
            // You'll need to pass these in from the client side
        ];

        let ix = Instruction {
            program_id: crate::ID,
            accounts,
            data: anchor_lang::InstructionData::data(
                &crate::instruction::ExecuteFlashloanSelfdump {
                    amount,
                    minimum_amount_out,
                    _is_buy,
                },
            ),
        };

        // Construct Solend's flash loan instruction with our callback
        let flashloan_ix = crate::solend::flash_loan_ix(
            ctx.accounts.lending_program.key(),
            ctx.accounts.reserve_liquidity_supply.key(),
            ctx.accounts.user_liquidity.key(),
            ctx.accounts.reserve.key(),
            ctx.accounts.lending_market_authority.key(),
            amount,
        );

        // Execute the flash loan via CPI
        invoke(
            &flashloan_ix,
            &[
                ctx.accounts.lending_program.to_account_info(),
                ctx.accounts.reserve.clone(),
                ctx.accounts.user_liquidity.clone(),
                ctx.accounts.reserve_liquidity_supply.clone(),
                ctx.accounts.lending_market_authority.clone(),
                ctx.accounts.token_program.to_account_info(),
            ],
        )?;

        Ok(())
    }

    // Your existing execute_flashloan_selfdump function remains here
    pub fn execute_flashloan_selfdump(
        ctx: Context<ExecuteFlashloanSelfdump>,
        amount: u64,
        minimum_amount_out: u64,
        _is_buy: bool,
    ) -> Result<()> {
        let whirlpool_account_info = &ctx.accounts.whirlpool;
        let whirlpool_data = whirlpool_account_info.try_borrow_data()?;

        let fee_tier: u16 = 64;

        let pdas = WhirlpoolPdas::new(
            &whirlpool_data,
            whirlpool_account_info.key(),
            ctx.accounts.whirlpool_program.key(),
            fee_tier,
        )?;

        // Construct the CPI Context for Whirlpool swap
        let cpi_accounts = whirlpool_cpi::cpi::accounts::Swap {
            whirlpool: ctx.accounts.whirlpool.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            token_authority: ctx.accounts.token_authority.to_account_info(),
            token_owner_account_a: ctx.accounts.token_owner_account.to_account_info(),
            token_vault_a: ctx.accounts.token_vault_a.to_account_info(),
            token_owner_account_b: ctx.accounts.user_liquidity.to_account_info(),
            token_vault_b: ctx.accounts.token_vault_b.to_account_info(),
            tick_array_0: ctx.accounts.tick_array_0.to_account_info(),
            tick_array_1: ctx.accounts.tick_array_1.as_ref().map(|a| a.to_account_info()).unwrap_or(ctx.accounts.tick_array_0.to_account_info()),
            tick_array_2: ctx.accounts.tick_array_2.as_ref().map(|a| a.to_account_info()).unwrap_or(ctx.accounts.tick_array_0.to_account_info()),
            oracle: ctx.accounts.oracle.to_account_info(),
        };

        let cpi_program = ctx.accounts.whirlpool_program.to_account_info();

        // Create the CPI Context
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Set swap parameters
        let sqrt_price_limit = 0; // 0 for no limit

        // Execute the swap via CPI
        whirlpool_cpi::cpi::swap(
            cpi_ctx,
            amount,
            minimum_amount_out,
            sqrt_price_limit,
            true,  // amount_specified_is_input
            true,  // a_to_b (direction of swap)
        )?;

        Ok(())
    }
}

// Update your ExecuteFlashloanSelfdump struct
#[derive(Accounts)]
pub struct ExecuteFlashloanSelfdump<'info> {
    // Solend Flash Loan Accounts
    /// SPL Token Lending program (Solend uses this under the hood)
    pub lending_program: Program<'info, FlashLoanProgram>,

    /// Reserve from which to borrow (USDC reserve)
    /// CHECK: This is validated by the lending program
    pub reserve: AccountInfo<'info>,

    /// Flashloan fee receiver
    /// CHECK: Not read by our program, just passed through
    pub reserve_liquidity_supply: AccountInfo<'info>,

    /// Lending market authority
    /// CHECK: Validated by Solend program
    pub lending_market_authority: AccountInfo<'info>,

    /// Destination for borrowed USDC
    #[account(mut)]
    pub user_liquidity: AccountInfo<'info>,

    // Orca Swap Accounts
    /// CHECK: Verified in CPI
    pub whirlpool_program: Program<'info, Whirlpool>,  // Changed from AccountInfo to Program
    /// CHECK: Verified in CPI
    #[account(signer)]
    pub token_authority: AccountInfo<'info>,
    /// CHECK: Verified in CPI
    #[account(mut)]
    pub token_owner_account: AccountInfo<'info>,
    /// CHECK: Verified in CPI
    #[account(mut)]
    pub token_vault_a: AccountInfo<'info>,
    /// CHECK: Verified in CPI
    #[account(mut)]
    pub token_vault_b: AccountInfo<'info>,
    /// CHECK: Verified in CPI
    pub token_mint_a: AccountInfo<'info>,
    /// CHECK: Verified in CPI
    pub token_mint_b: AccountInfo<'info>,
    /// CHECK: Verified in CPI
    #[account(mut)]
    pub whirlpool: AccountInfo<'info>,
    /// CHECK: Verified in CPI
    #[account(mut)]
    pub tick_array_0: AccountInfo<'info>,
    /// CHECK: Verified in CPI
    pub oracle: AccountInfo<'info>,
    /// Token program
    /// CHECK: Used in CPI
    pub token_program: AccountInfo<'info>,  // Changed from Program<'info, Token>
    /// CHECK: Verified in CPI
    #[account(mut)]
    pub fee_tier: AccountInfo<'info>,
    /// CHECK: Verified in CPI
    #[account(mut)]
    pub tick_array_1: Option<AccountInfo<'info>>,
    /// CHECK: Verified in CPI
    #[account(mut)]
    pub tick_array_2: Option<AccountInfo<'info>>,
}

#[derive(Clone)]
pub struct FlashLoanProgram;

impl anchor_lang::Id for FlashLoanProgram {
    fn id() -> Pubkey {
        use std::str::FromStr;
        Pubkey::from_str("LendZqTs7gn5CTSJU1jWKhKuVpjJGom45nnwPb2AMTi").unwrap()
    }
}