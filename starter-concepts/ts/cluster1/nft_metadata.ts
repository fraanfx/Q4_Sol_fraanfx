import wallet from "./wallet/wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

//change the endpoint from arweave.net to devnet.irys.xyz
//arweave link https://arweave.net/DXGjSeDUSazUCyQr55PDoNWcFLhqt94ordqv6f8qAAFD


(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const storageBaseUrl = 'https://devnet.irys.xyz/'
        const image = `${storageBaseUrl}DXGjSeDUSazUCyQr55PDoNWcFLhqt94ordqv6f8qAAFD`;
        const metadata = {
            name: "RugNft",
            symbol: "RN",
            description: "Rug NFT category golden",
            image: image,
            attributes: [
                {
                    trait_type: 'Background',
                    value: 'Golden',
                },
                {
                    trait_type: 'Center',
                    value: 'Emmerald',
                },
                {
                    trait_type: 'Rarity',
                    value: 'Legendary',
                },

            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image
                    },
                ]
            },
            creators: []
        };
        const myUri = await umi.uploader.uploadJson(metadata)
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
