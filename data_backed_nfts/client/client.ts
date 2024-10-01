// Client

// Log the wallet's public key
console.log("My address:", pg.wallet.publicKey.toString());

// Fetch and log the wallet's balance in SOL
const balance = await pg.connection.getBalance(pg.wallet.publicKey);
console.log(`My balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);

// Example of calling a custom method from your program
async function initializeDataAccount() {
  // Replace with own parameters
  const datasetId = "dataset123";
  const metadataUrl = "https://example.com/dataset-metadata";
  const bump = 123; // Example bump, in practice this is derived from PDA
  const expiry = new BN(Math.floor(Date.now() / 1000) + 60 * 60 * 24 * 30); // 30 days from now
  
  // Generate a new keypair for the data account
  const dataAccountKp = new web3.Keypair();

  // Use a valid Base58 public key for the NFT mint address
  const nftMint = new web3.PublicKey("YourValidNFTMintAddressHere"); // <-- Replace with valid public key

  // Initialize the data account by calling the initialize method from your program
  const txHash = await pg.program.methods
    .initialize(datasetId, metadataUrl, bump, expiry)
    .accounts({
      dataAccount: dataAccountKp.publicKey,
      nftMint: nftMint, // Make sure this is a valid Base58 public key
      mintAuthority: pg.wallet.publicKey,
      user: pg.wallet.publicKey,
      systemProgram: web3.SystemProgram.programId,
    })
    .signers([dataAccountKp])
    .rpc();

  console.log(`Data account initialized with transaction hash: ${txHash}`);
}

// Call the function to initialize the data account
await initializeDataAccount();
