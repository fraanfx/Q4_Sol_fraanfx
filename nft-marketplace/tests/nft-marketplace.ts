import { Program } from '@project-serum/anchor';
import { assert } from 'chai';
import { nft_marketplace } from '../target/types/nft_marketplace';
import { Keypair, PublicKey } from '@solana/web3.js';

describe('nft-marketplace', () => {
    const provider = anchor.Provider.local();
    anchor.setProvider(provider);

    const program = anchor.workspace.NftMarketplace as Program<nft_marketplace>;

    let marketplace: PublicKey;
    let initializer: Keypair;

    before(async () => {
        initializer = Keypair.generate();
        // Initialize the marketplace
        const tx = await program.rpc.initialize("My Marketplace", 100, {
            accounts: {
                initializer: initializer.publicKey,
                // ... other required accounts
            },
            signers: [initializer],
        });
        marketplace = tx; // Store the marketplace address
    });

    it('should list an NFT', async () => {
        // Test listing an NFT
        const tx = await program.rpc.list(new anchor.BN(1000), {
            accounts: {
                // ... required accounts for listing
            },
            signers: [initializer],
        });
        assert.ok(tx); // Check if the transaction was successful
    });

    it('should purchase an NFT', async () => {
        // Test purchasing an NFT
        const tx = await program.rpc.purchase({
            accounts: {
                // ... required accounts for purchasing
            },
            signers: [initializer],
        });
        assert.ok(tx); // Check if the transaction was successful
    });

    it('should delist an NFT', async () => {
        // Test delisting an NFT
        const tx = await program.rpc.delist({
            accounts: {
                // ... required accounts for delisting
            },
            signers: [initializer],
        });
        assert.ok(tx); // Check if the transaction was successful
    });
});