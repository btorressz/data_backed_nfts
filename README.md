# data_backed_nfts

This project implements a **Data-Backed NFT program** on the Solana blockchain using the **Anchor Framework**. The system allows the minting of NFTs that represent access to specific datasets, and it manages access control, leasing, and ownership transfer using **Program Derived Addresses (PDAs)**.
**THIS PROJECT IS A PROTOTYPE**

## Features
- **Mint NFTs** that represent access to datasets.
- **Access Control**: Only NFT holders can access premium datasets.
- **Leasing**: NFT holders can extend their access through leasing mechanisms.
- **Ownership Transfer**: Transfer the ownership of datasets to other users.
- **Fee-Based Access**: Users can be charged a fee (in SPL tokens) to access datasets.

  ### Key Files
- `lib.rs`: Contains the core logic for the program.
- `anchor.test.ts`: Unit tests for the program using Anchor's testing framework.
- `client.ts`: Example client script to interact with the deployed program.

  ## Tech Stack
  Rust, Typescript Solana, Anchor, Solana playground ide
