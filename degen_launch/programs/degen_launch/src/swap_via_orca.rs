use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
};

#[derive(Accounts)]
pub struct SwapViaOrca<'info> {
    /// CHECK: Verified in Orca CPI
    pub whirlpool_program: AccountInfo<'info>,
    
    /// CHECK: Token authority who can sign for the swap
    #[account(signer)]
    pub token_authority: AccountInfo<'info>,
    
    /// CHECK: Token account for input/output tokens
    #[account(mut)]
    pub token_owner_account: AccountInfo<'info>,
    
    /// CHECK: Vault for token A
    #[account(mut)]
    pub token_vault_a: AccountInfo<'info>,
    
    /// CHECK: Vault for token B 
    #[account(mut)]
    pub token_vault_b: AccountInfo<'info>,
    
    /// CHECK: Mint of token A
    pub token_mint_a: AccountInfo<'info>,
    
    /// CHECK: Mint of token B
    pub token_mint_b: AccountInfo<'info>,
    
    /// CHECK: The Whirlpool account
    #[account(mut)]
    pub whirlpool: AccountInfo<'info>,
    
    /// CHECK: First tick array account
    #[account(mut)]
    pub tick_array_0: AccountInfo<'info>,
    
    /// CHECK: Oracle account
    pub oracle: AccountInfo<'info>,
    
    /// Token Program
    pub token_program: AccountInfo<'info>,
    
    /// CHECK: Second tick array account (optional)
    #[account(mut)]
    pub tick_array_1: Option<AccountInfo<'info>>,
    
    /// CHECK: Third tick array account (optional)
    #[account(mut)]
    pub tick_array_2: Option<AccountInfo<'info>>,
    
    /// CHECK: Fee tier account
    #[account(mut)]
    pub fee_tier: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WhirlpoolSwapParams {
    amount: u64,
    other_amount_threshold: u64,
    sqrt_price_limit: u128,
    amount_specified_is_input: bool,
    a_to_b: bool,
}

impl<'info> SwapViaOrca<'info> {
    pub fn execute_swap(
        &self,
        amount_in: u64,
        minimum_amount_out: u64,
        is_a_to_b: bool,  // Add this parameter
    ) -> Result<()> {
        let mut account_metas = vec![
            AccountMeta::new(self.token_authority.key(), true),
            AccountMeta::new(self.token_owner_account.key(), false),
            AccountMeta::new(self.token_vault_a.key(), false),
            AccountMeta::new(self.token_vault_b.key(), false),
            AccountMeta::new(self.token_mint_a.key(), false),
            AccountMeta::new(self.token_mint_b.key(), false),
            AccountMeta::new(self.whirlpool.key(), false),
            AccountMeta::new(self.tick_array_0.key(), false),
            AccountMeta::new(self.oracle.key(), false),
            AccountMeta::new(self.token_program.key(), false),
            AccountMeta::new(self.fee_tier.key(), false),
        ];

        // Add optional tick arrays if they exist
        if let Some(tick_array_1) = &self.tick_array_1 {
            account_metas.push(AccountMeta::new(tick_array_1.key(), false));
        }
        if let Some(tick_array_2) = &self.tick_array_2 {
            account_metas.push(AccountMeta::new(tick_array_2.key(), false));
        }

        let ix = Instruction {
            program_id: self.whirlpool_program.key(),
            accounts: account_metas,
            data: self.build_swap_instruction_data(amount_in, minimum_amount_out, is_a_to_b),
        };

        let mut account_infos = vec![
            self.whirlpool_program.to_account_info(),
            self.token_authority.to_account_info(),
            self.token_owner_account.to_account_info(),
            self.token_vault_a.to_account_info(),
            self.token_vault_b.to_account_info(),
            self.token_mint_a.to_account_info(),
            self.token_mint_b.to_account_info(),
            self.whirlpool.to_account_info(),
            self.tick_array_0.to_account_info(),
            self.oracle.to_account_info(),
            self.token_program.to_account_info(),
            self.fee_tier.to_account_info(),
        ];

        // Add optional tick arrays to account_infos
        if let Some(tick_array_1) = &self.tick_array_1 {
            account_infos.push(tick_array_1.to_account_info());
        }
        if let Some(tick_array_2) = &self.tick_array_2 {
            account_infos.push(tick_array_2.to_account_info());
        }

        invoke(
            &ix,
            &account_infos
        )?;

        Ok(())
    }

    fn build_swap_instruction_data(&self, amount_in: u64, min_out: u64, is_a_to_b: bool) -> Vec<u8> {
        const SWAP_IX: u8 = 1;

        let params = WhirlpoolSwapParams {
            amount: amount_in,
            other_amount_threshold: min_out,
            sqrt_price_limit: 0,
            amount_specified_is_input: true,
            a_to_b: is_a_to_b,
        };

        let mut data = vec![SWAP_IX];
        data.extend_from_slice(&params.try_to_vec().unwrap());
        data
    }
}