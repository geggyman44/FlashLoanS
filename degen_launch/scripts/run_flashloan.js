require('dotenv').config();
const anchor = require("@coral-xyz/anchor");
const { PublicKey, Connection } = require("@solana/web3.js");
const path = require('path');

// Define addresses constants
const ADDRESSES = {
    LENDING_PROGRAM: "LendZqTs7gn5CTSJU1jWKhKuVpjJGom45nnwPb2AMTi",
    USDC_RESERVE: "BgxfHJDzm44T7XG68MYKx7YisTjZu73tVovyZSjJMpmw",
    // Add other addresses as needed
};

// Initialize connection
const connection = new Connection(
    process.env.ANCHOR_PROVIDER_URL || "https://api.mainnet-beta.solana.com",
    "confirmed"
);

// Initialize provider once
const wallet = anchor.Wallet.local();
const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "confirmed"
});

// Set the provider before creating program
anchor.setProvider(provider);

async function initializeProgram() {
    try {
        console.log("Initializing program...");
        const idlPath = path.join(__dirname, '../target/idl/degen_launch.json');
        const idl = require(idlPath);
        console.log("IDL loaded successfully");

        const programId = new PublicKey("6UBFGLf5YBdVAzdzzoMhQsL3pM1KjgRp7EgVDCP4UqGV");
        return anchor.Program.at(programId, provider);
    } catch (error) {
        console.error("Program initialization failed:", error);
        throw error;
    }
}

async function executeFlashLoan() {
    console.log("Starting flash loan execution...");
    try {
        const program = await initializeProgram();
        if (!program) {
            throw new Error("Program initialization failed");
        }

        // Set up accounts
        const accounts = {
            lendingProgram: new PublicKey(ADDRESSES.LENDING_PROGRAM),
            reserve: new PublicKey(ADDRESSES.USDC_RESERVE),
            // Add other accounts...
        };

        console.log("Executing flash loan transaction...");
        const tx = await program.methods
            .executeFlashloanSelfdump(
                new anchor.BN(1000000000),
                new anchor.BN(990000000),
                true
            )
            .accounts(accounts)
            .rpc();

        console.log("Transaction successful!");
        console.log("Signature:", tx);
        return tx;
    } catch (error) {
        console.error("Flash loan execution failed:", error);
        if (error.logs) {
            console.error("Transaction logs:", error.logs);
        }
        process.exit(1);
    }
}

// Actually execute the function
console.log("Script started");
executeFlashLoan()
    .then(() => {
        console.log("Script completed successfully");
        process.exit(0);
    })
    .catch((error) => {
        console.error("Script failed:", error);
        process.exit(1);
    });