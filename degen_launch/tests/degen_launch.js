const anchor = require("@coral-xyz/anchor");
const { PublicKey, TOKEN_PROGRAM_ID } = require("@solana/web3.js");
const { getWhirlpoolPDAs } = require('@orca-so/whirlpools-sdk');
const { Token } = require("@solana/spl-token");

describe("degen_launch", () => {
  // Get the provider and program
  anchor.setProvider(anchor.AnchorProvider.env());
  
  // Load the IDL from the workspace
  const idl = require('../target/idl/degen_launch.json');
  const programId = new PublicKey('6UBFGLf5YBdVAzdzzoMhQsL3pM1KjgRp7EgVDCP4UqGV');
  const program = new anchor.Program(idl, programId);

  // Rest of your code remains the same...
  const ADDRESSES = {
    LENDING_PROGRAM: "LendZqTs7gn5CTSJU1jWKhKuVpjJGom45nnwPb2AMTi",
    USDC_RESERVE: "BgxfHJDzm44T7XG68MYKx7YisTjZu73tVovyZSjJMpmw",
    RESERVE_LIQUIDITY: "8SheGtsopRUDzdiD6v6BR9a6bqZ9QwywYQY99Fp5meNf",
    LENDING_AUTHORITY: "DdZR6zRFiUt4S5mg7AV1uKB2z1f1WzcNYCaTEEWPAuby",
    USER_USDC: "FXsHUq4au79ZLmA9FiXB44cDJKzxHgWuPnLij816kHPZ",
    DEGEN_MINT: "E1YrvUKxBzrx5mGtL4D3VB12Jxr8xn1ibpZq6XCGX7es",
    USER_DEGEN: "E6dvNzzQ8cztRPzhLo4mRy9hhSywxHZQx9kzqdzUTeY5",
    USER_WALLET: "GHLkM2szK28hwq2uLYH361euMUUw441Ec3TZJf39owA6",
    USDC_MINT: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v" // Added USDC_MINT
  };

  it("Execute flash loan + self dump", async () => {
    try {
      const flashLoanAmount = new anchor.BN(1000000000); // 1000 USDC
      const minimumSwapAmountOut = new anchor.BN(990000000); // 990 USDC (1% slippage)

      // Get Whirlpool PDAs
      const whirlpoolPDAs = await getWhirlpoolPDAs(
        new PublicKey(ADDRESSES.DEGEN_MINT),  // tokenMintA
        new PublicKey(ADDRESSES.USDC_MINT),   // tokenMintB
        64,  // fee tier (example: 0.0064%)
        new PublicKey("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc")  // whirlpool program
      );

      const tx = await program.methods
        .executeFlashloanSelfdump(
          flashLoanAmount,
          minimumSwapAmountOut,
          true
        )
        .accounts({
          lendingProgram: new PublicKey(ADDRESSES.LENDING_PROGRAM),
          reserve: new PublicKey(ADDRESSES.USDC_RESERVE),
          reserveLiquiditySupply: new PublicKey(ADDRESSES.RESERVE_LIQUIDITY),
          lendingMarketAuthority: new PublicKey(ADDRESSES.LENDING_AUTHORITY),
          userLiquidity: new PublicKey(ADDRESSES.USER_USDC),
          tokenProgram: TOKEN_PROGRAM_ID, // Use TOKEN_PROGRAM_ID from @solana/web3.js
          whirlpoolProgram: new PublicKey("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"),
          tokenAuthority: new PublicKey(ADDRESSES.USER_WALLET),
          tokenOwnerAccount: new PublicKey(ADDRESSES.USER_DEGEN),
          whirlpool: whirlpoolPDAs.whirlpool,
          oracle: whirlpoolPDAs.oracle,
          tickArray0: whirlpoolPDAs.tickArray0,
          tokenVaultA: whirlpoolPDAs.tokenVaultA, // You'll need to add these from whirlpoolPDAs
          tokenVaultB: whirlpoolPDAs.tokenVaultB,
          tokenMintA: new PublicKey(ADDRESSES.DEGEN_MINT),
          tokenMintB: new PublicKey(ADDRESSES.USDC_MINT),
          feeTier: whirlpoolPDAs.feeTier,
          tickArray1: whirlpoolPDAs.tickArray1 || null,
          tickArray2: whirlpoolPDAs.tickArray2 || null
        })
        .rpc();
      
      console.log("Transaction signature:", tx);
    } catch (error) {
      console.error("Error:", error);
      throw error;
    }
  });
});