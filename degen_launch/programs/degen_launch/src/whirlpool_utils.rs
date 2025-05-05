use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Whirlpool account data.")]
    InvalidWhirlpoolAccount,
}

pub fn parse_tick_index_from_whirlpool(data: &[u8]) -> Result<i32> {
    if data.len() < 77 {
        return Err(ErrorCode::InvalidWhirlpoolAccount.into());
    }

    // tick_current_index starts at offset 73 and is 4 bytes long
    let tick_bytes = &data[73..77];
    let tick_array: [u8; 4] = tick_bytes.try_into().unwrap();
    Ok(i32::from_le_bytes(tick_array))
}

// Keep the TICK_ARRAY_SIZE constant
const TICK_ARRAY_SIZE: i32 = 88;

// Keep the existing derive_tick_arrays function
pub fn derive_tick_arrays(
    whirlpool: &Pubkey,
    whirlpool_program: &Pubkey,
    current_tick_index: i32,
) -> (Pubkey, Pubkey, Pubkey) {
    // Calculate base tick index
    let start_tick_index = (current_tick_index / TICK_ARRAY_SIZE) * TICK_ARRAY_SIZE;
    
    // Derive tick_array_0 (current)
    let (tick_array_0, _) = Pubkey::find_program_address(
        &[
            b"tick_array",
            whirlpool.as_ref(),
            &start_tick_index.to_le_bytes(),
        ],
        whirlpool_program,
    );

    // Derive tick_array_1 (next)
    let (tick_array_1, _) = Pubkey::find_program_address(
        &[
            b"tick_array",
            whirlpool.as_ref(),
            &(start_tick_index + TICK_ARRAY_SIZE).to_le_bytes(),
        ],
        whirlpool_program,
    );

    // Derive tick_array_2 (previous)
    let (tick_array_2, _) = Pubkey::find_program_address(
        &[
            b"tick_array",
            whirlpool.as_ref(),
            &(start_tick_index - TICK_ARRAY_SIZE).to_le_bytes(),
        ],
        whirlpool_program,
    );

    (tick_array_0, tick_array_1, tick_array_2)
}

pub struct WhirlpoolPdas {
    pub whirlpool: Pubkey,
    pub oracle: Pubkey,
    pub tick_array_0: Pubkey,
    pub tick_array_1: Option<Pubkey>,
    pub tick_array_2: Option<Pubkey>,
    pub fee_tier: Pubkey,
}

impl WhirlpoolPdas {
    pub fn new(
        whirlpool_account_data: &[u8],
        whirlpool: Pubkey,
        whirlpool_program: Pubkey,
        fee_tier_value: u16, // Add this parameter
    ) -> Result<Self> {
        let current_tick_index = parse_tick_index_from_whirlpool(whirlpool_account_data)?;

        let (oracle, _) = Pubkey::find_program_address(
            &[b"oracle", whirlpool.as_ref()],
            &whirlpool_program,
        );

        let (tick_array_0, tick_array_1, tick_array_2) =
            derive_tick_arrays(&whirlpool, &whirlpool_program, current_tick_index);

        let (fee_tier, _) = Pubkey::find_program_address(
            &[b"fee_tier", &fee_tier_value.to_le_bytes()],
            &whirlpool_program,
        );

        Ok(Self {
            whirlpool,
            oracle,
            tick_array_0,
            tick_array_1: Some(tick_array_1),
            tick_array_2: Some(tick_array_2),
            fee_tier,
        })
    }
}

// Update derive_whirlpool_pdas to pass the fee_tier value
pub fn derive_whirlpool_pdas(
    whirlpool_program: &Pubkey,
    token_mint_a: &Pubkey,
    token_mint_b: &Pubkey,
    fee_tier: u16,
    whirlpool_data: Option<&[u8]>,
) -> Result<WhirlpoolPdas> {
    let (whirlpool, _) = Pubkey::find_program_address(
        &[
            b"whirlpool",
            token_mint_a.as_ref(),
            token_mint_b.as_ref(),
            &fee_tier.to_le_bytes(),
        ],
        whirlpool_program,
    );

    if let Some(data) = whirlpool_data {
        // Pass the fee_tier value to new()
        WhirlpoolPdas::new(data, whirlpool, *whirlpool_program, fee_tier)
    } else {
        let (oracle, _) = Pubkey::find_program_address(
            &[b"oracle", whirlpool.as_ref()],
            whirlpool_program,
        );

        let (fee_tier_pda, _) = Pubkey::find_program_address(
            &[b"fee_tier", &fee_tier.to_le_bytes()],
            whirlpool_program,
        );

        Ok(WhirlpoolPdas {
            whirlpool,
            oracle,
            tick_array_0: Pubkey::default(),
            tick_array_1: None,
            tick_array_2: None,
            fee_tier: fee_tier_pda,
        })
    }
}