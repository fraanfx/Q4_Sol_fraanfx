import wallet from "./wallet/wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import { sign } from "crypto";
import { PublicKey } from "@solana/web3.js";
import bs58 from "bs58"; // Import bs58 for encoding the signature


// Define our Mint address
const mint = publicKey("EVZFZgc1cJ6EeZNnxdv2rQSvouMzHwwEQUQZRLW3TnLj");

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');

// Create a signer from the local wallet
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

// Use the signer identity for transactions
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));



(async () => {
    try {
        // Start here
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            mint: mint,
            mintAuthority: signer,
            payer: signer,
            updateAuthority: signer.publicKey,
        }

        // Metadata for the SPL
        let data: DataV2Args = {
            name: "NewTokenTest",
            symbol: "NTT",
            uri: "https://metadata-url.com/metadata.json", // link to off-chain metadata
            sellerFeeBasisPoints: 500, // Royalties (500 = 5%)
            creators: null, // Optional: Array of creator public keys
            collection: null, // Optional: NFT collection details
            uses: null, // Optional: Use case restrictions
          };

        let args: CreateMetadataAccountV3InstructionArgs = {
            data: data,
            isMutable: true, // Can be updated later
            collectionDetails: null,
        }

        let tx = createMetadataAccountV3(
            umi,
            {
                ...accounts,
                ...args
            }
        )

        let result = await tx.sendAndConfirm(umi);
        // console.log(bs58.encode(result.signature));
        console.log(umi.transactions.deserialize(result.signature))
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
