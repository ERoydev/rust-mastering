
Good Blog: https://medium.com/@praptii/solana-account-model-pdas-explained-754319582462

## Types of Accounts in Solana:

### Program Accounts
These store compiled executable code. They are like proxy.

### Program Data Accounts
Store data associated with the program (like state). Ruust code is stored here.

### User Accounts
Wallets controlled by users, usually holding SOL.

### Data Accounts
Used to store any arbitrary data, usually controlled by a program. Here we can dump data, bunch of bytes.

### Program Derived Addresses (PDAs)
Special accounts deterministically derived and only signable by programs.
