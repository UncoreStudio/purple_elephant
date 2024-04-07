import web3, { sendAndConfirmTransaction } from "@solana/web3.js"
import fs from "node:fs"
import path from "path"

// Créer un client RPC pour interagir avec le réseau Solana
const rpcUrl = 'http://127.0.0.1:8899'; // Remplacez par l'URL du nœud RPC Solana
const rpc = new web3.Connection(rpcUrl);

// Remplacer ces valeurs avec les clés publiques des comptes impliqués
const programId = new web3.PublicKey('GYLrDmuv4bmF7p3kRJrFYAxpx6zaPHcjwJULuTEERbtP'); // ID de votre programme Solana
const liquidityPoolAccountPubkey = web3.Keypair.generate(); // ID de la pool de liquidité

const keypairConfig = JSON.parse(fs.readFileSync(path.join(process.cwd(), '../.config/solana/id.json'), 'utf8'));
const keypair = web3.Keypair.fromSecretKey(
    Uint8Array.from(keypairConfig)
);

const userAccountPubkey = new web3.PublicKey(keypair.publicKey);

// Créer une instruction pour déposer dans la pool de liquidité

const instruction = new web3.TransactionInstruction({
    programId,
    keys: [
        { pubkey: liquidityPoolAccountPubkey.publicKey, isSigner: false, isWritable: true },
        { pubkey: userAccountPubkey, isSigner: true, isWritable: true },
    ],
    data: instructionData,
});

// Créer une transaction pour envoyer l'instruction
const transaction = new web3.Transaction().add(instruction);

// Signer la transaction avec la clé privée du compte payeur
const result = await rpc.getLatestBlockhash();

transaction.recentBlockhash = result.blockhash;
const from = web3.Keypair.fromSecretKey(
    Buffer.from(fs.readFileSync(path.join(process.cwd(), '../.config/solana/id.json'), 'utf-8'))
)
sendAndConfirmTransaction(rpc, transaction, [from]).then(result => {
    console.log('Transaction envoyée:', result);
}).catch(error => {
    console.error('Erreur lors de l\'envoi de la transaction:', error);
});