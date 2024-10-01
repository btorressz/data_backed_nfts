// No imports needed: web3, anchor, pg, and more are globally available

describe("Data-Backed NFT Tests", () => {
  it("initialize data account", async () => {
    // Generate keypair for the new data account
    const dataAccountKp = new web3.Keypair();

    // Example test data for the dataset initialization
    const datasetId = "dataset123";
    const metadataUrl = "https://example.com/dataset-metadata";
    const bump = 123; // Example bump, in practice it would come from PDA
    const expiry = new BN(Math.floor(Date.now() / 1000) + 60 * 60 * 24 * 30); // 30 days from now

    // Send transaction to initialize the data account
    const txHash = await pg.program.methods
      .initialize(datasetId, metadataUrl, bump, expiry)
      .accounts({
        dataAccount: dataAccountKp.publicKey, // Newly created data account
        nftMint: new web3.PublicKey("NFT_MINT_ADDRESS_HERE"), // Replace with actual NFT mint
        mintAuthority: pg.wallet.publicKey,
        user: pg.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([dataAccountKp]) // Sign the transaction with the new account keypair
      .rpc();

    console.log(`Transaction submitted with hash: ${txHash}`);
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm the transaction
    await pg.connection.confirmTransaction(txHash);

    // Fetch the created data account
    const dataAccount = await pg.program.account.dataAccount.fetch(
      dataAccountKp.publicKey
    );

    // Verify the dataset ID, metadata URL, and expiry
    console.log("On-chain data is:", dataAccount.datasetId, dataAccount.metadataUrl, dataAccount.expiry.toString());

    // Assertions to ensure data is correctly initialized on-chain
    assert.equal(dataAccount.datasetId, datasetId);
    assert.equal(dataAccount.metadataUrl, metadataUrl);
    assert(dataAccount.expiry.eq(expiry));

    console.log("Data account initialization successful");
  });
});
