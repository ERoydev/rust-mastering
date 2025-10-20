# Sending a Solana Transaction in JavaScript

This is a concise guide to what you need when sending a transaction on Solana using JavaScript.

---

## 1. Connection / Provider
**What it is:** The object that connects your JS app to the blockchain.

**Example:**
import { Connection, clusterApiUrl } from '@solana/web3.js';

const connection = new Connection(clusterApiUrl('devnet'));

**Why you need it:** All transaction info (blockhash, account info, balances) comes through the provider/connection.

---

## 2. Payer / Signer
**What it is:** The account that pays transaction fees and signs the transaction.

**Example:**
import { Keypair } from '@solana/web3.js';

const payer = Keypair.generate(); // Or load from secret key

**Why you need it:** Without a signer, the blockchain will reject your transaction.

---

## 3. Recent Blockhash
**What it is:** A hash of a recent block, used to prevent replay attacks and expire old transactions.

**Example:**
const { blockhash } = await connection.getLatestBlockhash();

**Why you need it:** Every transaction must reference a recent blockhash to be valid.

---

## 4. Instructions / Message
**Instruction:** A single blockchain action (e.g., token transfer).  
**Message:** Encapsulates all instructions + fee payer + recent blockhash.

**Example:**
import { Transaction, SystemProgram } from '@solana/web3.js';

const transaction = new Transaction().add(
    SystemProgram.transfer({
        fromPubkey: payer.publicKey,
        toPubkey: recipientPublicKey,
        lamports: 1000,
    })
);

transaction.recentBlockhash = blockhash;
transaction.feePayer = payer.publicKey;

**Why you need it:** The blockchain needs instructions to know what action to perform.

---

## 5. Sign the Transaction
**What it is:** Authorizing the transaction with your payer/signers.

**Example:**
transaction.sign(payer);

**Why you need it:** Unsigned transactions are rejected.

---

## 6. Send the Transaction
**What it is:** Broadcasting the signed transaction to the blockchain.

**Example:**
import { sendAndConfirmTransaction } from '@solana/web3.js';

await sendAndConfirmTransaction(connection, transaction, [payer]);

**Why you need it:** This sends your transaction and waits for confirmation.

---

## ✅ Summary
Minimal things you need to send a Solana transaction from JS:

1. **Connection** → connect to the blockchain  
2. **Payer/Signer** → authorize and pay fees  
3. **Recent Blockhash** → prevent replay attacks  
4. **Transaction / Instructions / Message** → define the action  
5. **Sign** → authorize transaction  
6. **Send** → broadcast transaction
