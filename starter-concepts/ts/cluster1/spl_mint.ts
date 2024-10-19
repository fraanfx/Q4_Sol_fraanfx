import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import wallet from "./wallet/wba-wallet.json"

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n; //0s provided on create mint

// Mint address
//VcaR12KU15JLcyrzF7bjxAJNr4n3aCTfgkLk6cjEnut
const mint = new PublicKey("EVZFZgc1cJ6EeZNnxdv2rQSvouMzHwwEQUQZRLW3TnLj");

(async () => {
    try {
        // Create an ATA account for the token mint account provided from spl_init.ts
        const ata = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey);
        console.log(`Your ata is: ${ata.address.toBase58()}`);

        // Mint from token address to ATA 
        const mintTx = await mintTo(connection, keypair, mint, ata.address, keypair, 10000n * token_decimals)
        console.log(`Your mint txid: ${mintTx}`);

        const ata2 =  await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey)
        console.log(`Your balance is: ${mintTx}`);
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
