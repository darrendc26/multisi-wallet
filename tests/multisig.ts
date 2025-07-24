import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Multisig } from "../target/types/multisig";
import { PublicKey, Keypair } from "@solana/web3.js";
import { assert } from "chai";

let multisig: PublicKey;
let creatorKeypair: Keypair; 
let owner1Keypair: Keypair;
let owners: PublicKey[] = [];
let threshold: number = 2;
let transaction: PublicKey;

// Generate Keypairs (not just PublicKeys)
creatorKeypair = anchor.web3.Keypair.generate();
owner1Keypair = anchor.web3.Keypair.generate();
let owner2Keypair = anchor.web3.Keypair.generate();
let owner3Keypair = anchor.web3.Keypair.generate();
let owner4Keypair = anchor.web3.Keypair.generate();

owners = [
  creatorKeypair.publicKey,
  owner1Keypair.publicKey,
  owner2Keypair.publicKey,
  owner3Keypair.publicKey,
  owner4Keypair.publicKey
];

before(async () => {
  const connection = anchor.getProvider().connection;
  const airdropSignature = await connection.requestAirdrop(
    creatorKeypair.publicKey,
    anchor.web3.LAMPORTS_PER_SOL // 1 SOL
  );
  await connection.confirmTransaction({
    signature: airdropSignature,
    blockhash: (await connection.getLatestBlockhash()).blockhash,
    lastValidBlockHeight: (await connection.getLatestBlockhash()).lastValidBlockHeight,
  });
  console.log("", creatorKeypair.publicKey.toString());

  const airdropSignature2 = await connection.requestAirdrop(
    owner1Keypair.publicKey,
    anchor.web3.LAMPORTS_PER_SOL // 1 SOL
  );
  await connection.confirmTransaction({
    signature: airdropSignature2,
    blockhash: (await connection.getLatestBlockhash()).blockhash,
    lastValidBlockHeight: (await connection.getLatestBlockhash()).lastValidBlockHeight,
  });

  console.log("Owner 1 airdropped:", owner1Keypair.publicKey.toString());
});

describe("multisig", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.multisig as Program<Multisig>;

  it("Creates a multisig account", async () => {
    const [multisigAccount, bump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("multisig"), creatorKeypair.publicKey.toBuffer()],
      program.programId
    );

    try {
      await program.methods
        .createMultisig(owners, threshold)
        .accounts({
          // multisig: multisigAccount,
          owner: creatorKeypair.publicKey,
          // systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([creatorKeypair]) // Add the actual keypair as signer
        .rpc();

      // Check that the multisig account was created
      const multisigAccountData = await program.account.multisig.fetch(multisigAccount);
      
      assert.ok(multisigAccountData.creator.equals(creatorKeypair.publicKey)); // Should equal the creator's public key
      assert.equal(multisigAccountData.owners.length, 5); // Should equal the number of owners
      assert.equal(multisigAccountData.threshold, threshold); // Should equal the threshold
      assert.equal(multisigAccountData.nonce, 0); // Should start at 0
      assert.equal(multisigAccountData.bump, bump); // Should equal the PDA bump

      multisig = multisigAccount;
      console.log("Multisig created successfully:", multisig.toString());

    } catch (err) {
      console.error("Error details:", err);
      throw err; // Re-throw to fail the test
    }
  });

  it("Proposes a transaction", async () => {
    const multisigData = await program.account.multisig.fetch(multisig);
    const nonce = multisigData.nonce;
    const nonceBytes = Buffer.alloc(2);
    nonceBytes.writeUInt16LE(nonce, 0);
    const [transactionAccount, bump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("transaction"), multisig.toBuffer(), nonceBytes],
      program.programId
    );

    const instruction = [
      {
        programId: PublicKey.default,
        accounts: [
          {
            pubkey: PublicKey.default,
            isSigner: true,
            isWritable: true,
          },
        ],
        data: Buffer.from("test"),
      },
    ];

    try {
      await program.methods.proposeTxn(instruction)
      .accounts({
        multisig: multisig,
        proposer: owner1Keypair.publicKey,
      })
      .signers([owner1Keypair])
      .rpc();

      // Check that the transaction was created
      const transactionAccountData = await program.account.transaction.fetch(transactionAccount);
      assert.equal(transactionAccountData.executed, false);
      assert.equal(transactionAccountData.multisig.toString(), multisig.toString());
      assert.equal(transactionAccountData.proposer.toString(), owner1Keypair.publicKey.toString());
      assert.equal(transactionAccountData.signers.length, 1);
      assert.equal(transactionAccountData.signers[0].toString(), owner1Keypair.publicKey.toString());
      console.log("Transaction created successfully:", transactionAccount.toString());
      console.log("Txn signers:  ", transactionAccountData.signers);
    } catch (err) {
      console.error("Error details:", err);
    }
  });

});



