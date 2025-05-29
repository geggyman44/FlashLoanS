require('dotenv').config();
const anchor = require("@coral-xyz/anchor");
const { PublicKey, Connection } = require("@solana/web3.js");
const path = require('path');
const fs = require('fs');

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

        // Check if IDL file exists
        if (!fs.existsSync(idlPath)) {
            throw new Error(`IDL file not found at ${idlPath}. Make sure you've run 'anchor build' first.`);
        }

        console.log("Reading IDL file from:", idlPath);
        const idlContent = fs.readFileSync(idlPath, 'utf8');
        const idl = JSON.parse(idlContent);

        console.log("IDL loaded successfully");
        console.log("IDL keys:", Object.keys(idl));

        // Since your IDL doesn't have an address field, use the hardcoded program ID
        const programIdString = "6UBFGLf5YBdVAzdzzoMhQsL3pM1KjgRp7EgVDCP4UqGV";
        console.log("Using hardcoded program ID:", programIdString);

        console.log("Creating PublicKey from:", programIdString);
        const programId = new PublicKey(programIdString);
        console.log("Program ID created successfully:", programId.toString());

        // Add the address to the IDL object if it's missing
        if (!idl.address && !idl.metadata) {
            idl.address = programIdString;
            console.log("Added address to IDL");
        }

        // Create program instance with explicit parameters
        console.log("Creating Program instance...");
        const program = new anchor.Program(idl, programId, provider);
        console.log("Program initialized successfully");

        return program;
    } catch (error) {
        console.error("Program initialization failed:", error);
        console.error("Error stack:", error.stack);
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

        console.log("Program methods available:", Object.keys(program.methods));

        // Set up accounts
        const accounts = {
            lendingProgram: new PublicKey(ADDRESSES.LENDING_PROGRAM),
            reserve: new PublicKey(ADDRESSES.USDC_RESERVE),
            // Add other accounts...
        };

        console.log("Executing flash loan transaction...");

        // Check if the method exists before calling it
        if (!program.methods.executeFlashloanSelfdump) {
            throw new Error("Method 'executeFlashloanSelfdump' not found in program. Available methods: " + Object.keys(program.methods).join(', '));
        }

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
        throw error;
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