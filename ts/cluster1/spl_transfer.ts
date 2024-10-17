import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "./wallet/wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, TOKEN_2022_PROGRAM_ID, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("VcaR12KU15JLcyrzF7bjxAJNr4n3aCTfgkLk6cjEnut");

// Recipient address
const to = new PublicKey("F5VWfu8Emo3jWgWvFidhfnXGaM8WZ4AmHUhuiHhnAM2R");
const TOKEN_DECIMALS =  1_000_000n;

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey,
        )

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to,
        )

        // Transfer the new token to the "toTokenAccount" we just created
        const signature =  await transfer(
            connection,
            keypair,
            fromTokenAccount.address,
            toTokenAccount.address,
            keypair,
            10n * TOKEN_DECIMALS,
        )
        console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`);

    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();