import { config } from "dotenv";
import { Connection, clusterApiUrl, Keypair, PublicKey, TransactionInstruction, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import bs58 from "bs58"

config(); // load .env

// 1. connection
const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
const programId = new PublicKey("9eX2VoYXNnKbaS2HmhjgsLMpETTnvL9Tgjg2Wt6rGR1e");

// 2. payer, Load the keypair from my phantom wallet secret key
const keyString = process.env.PRIVATE_KEY!;
const secretKey = bs58.decode(keyString);
const payer = Keypair.fromSecretKey(secretKey);

// 3. Recent blockhash
const { blockhash } = await connection.getLatestBlockhash("finalized");

/*
r2 = *(u64 *)(r1 + 0x0)

if r2 != 0x0 goto +0x15 -> conditional jump
- if r2 is non-zero, the program jumps ahead by 0x15 bytes
- if r2 is zero, program continues without jumping
*/

const instruction_data = Buffer.alloc(32);
instruction_data.writeBigUInt64LE(6n, 0);  // offset 0x0 = 0
instruction_data.writeBigUInt64LE(12n, 8);  // offset 0x8 = 8
instruction_data.writeBigUInt64LE(23n, 16); // offset 0x10 = 16
instruction_data.writeBigUInt64LE(5n, 24); // offset 0x18 = 24

// Offset start from a pointer x, then move that many bytes, tells where to read or write data
// With offset i just specify where each u64 value starts in the buffer

const instruction = new TransactionInstruction({
    keys: [],
    programId,
    data: instruction_data
})

// 4. Create transaction
const transaction = new Transaction();
transaction.add(instruction);

const signature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [payer]
);

console.log("Transaction signature", signature);