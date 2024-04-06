import web3, { Connection, Keypair, Transaction, TransactionInstruction } from '@solana/web3.js';
import fs from "node:fs";
let connection = new Connection("https://api.devnet.solana.com", "confirmed");

const triggerKeyPair = Keypair.generate();

const transaction = new Transaction({
    feePayer: triggerKeyPair.publicKey,
})


const makeTransaction = async (programId, keyPairFile) => {
    // Create a array of keys
    const borrowerKeyPair = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync(keyPairFile))))

    transaction.add(
        new TransactionInstruction({
            keys: [{ pubkey: borrowerKeyPair.publicKey, isSigner: true, isWritable: true }],
            programId,
            data: Buffer.alloc(0), // All instructions are empty
        }),
        
    )
    await web3.sendAndConfirmTransaction(connection, transaction, [
        Keypair.generate(),
        keyPairFile,
      ]);
    console.log('Transaction confirmed');
}

const programId = new Keypair('GYLrDmuv4bmF7p3kRJrFYAxpx6zaPHcjwJULuTEERbtP');

makeTransaction(programId, '/home/azones/.config/solana/id.json');

