import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "./wallet/wba-wallet.json"
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

// NFT Metadata https://arweave.net/7qgMrvQTnvW37b7YbTPZScjo5y8QTXqFcw7hyZtoDh3R

(async () => {

    const storageBaseUrl = 'https://devnet.irys.xyz/'
     let tx = await createNft(umi, 
        { mint, sellerFeeBasisPoints:  percentAmount(5), name: "Rug", symbol: "RN", uri: `${storageBaseUrl}7qgMrvQTnvW37b7YbTPZScjo5y8QTXqFcw7hyZtoDh3R`}
    );

    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();