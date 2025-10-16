# Indexing
Blod post: https://medium.com/@praptii/indexing-on-solana-a-complete-guide-to-deposits-withdrawals-memos-and-security-4ecb2d2f3f69

- Indexing is simply taking some specific `transaction_data` from the blockchain(on-chain) and saving it in a database, the idea is to take only data that is relevant to the dApp we have.

So we often focus on indexing specific addresses or transactions relevant to our application:
- All transactions to our exchange’s deposit address
- Activity on specific liquidity pools
- Transactions involving particular token contracts
- User deposits and withdrawals



The most robust approach to Solana indexing is using `Yellowstone`, built on top of the `Geyser` plugin. These tools allow us to subscribe to real-time blockchain events as they happen.

When validators on the Solana network receive transactions and create blocks, they can run additional processes (plugins) that extract and forward specific transaction data. Yellowstone provides a gRPC interface to these plugins, enabling us to:
- Subscribe to specific accounts or programs
- Filter transactions based on our criteria
- Receive real-time updates as transactions are confirmed

