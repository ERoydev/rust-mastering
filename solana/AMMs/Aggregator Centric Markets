## The Aggregator-Centric Market

---

### Ethereum vs Solana market structure
On Solana, users mostly interact with aggregators (like Jupiter), which automatically route swaps across multiple DEXes, rather than interacting with individual DEXs directly.

**EVM:**
- Users often interact directly with individual DEXs like Uniswap, SushiSwap or Curve
- Each DEX has its own interface and liquidity.
- Aggregators like 1inch exist, but they’re optional — many users just go to one DEX.

**SVM:**
- The architecture allows atomic composition of multiple instructions in a single transaction -> Meaning: a single transaction can execute multiple swaps across multiple AMMs in one go.
- This makes it easy to have aggregators that handle multiple DEXs for the user automatically.
- Result: the market is aggregator-centric — most users go through aggregators rather than individual DEXs.

**Aggregator-centric means:**
- Users don’t have to pick Radium, Orca, or Jupiter themselves.
- They interact with one entry point (aggregator), which routes the swap across the best liquidity sources.

**Example:**
User → Phantom Wallet → Jupiter → (Raydium + Orca + other pools) -> Users sees one swap, aggregator figures out the best path across all available liquidity.

**Jupiter:**
- Is the primary aggregator on Solana.

---

### Why Solana evolved this way
* SVM allows atomic multi-instruction transactions → routers can execute swaps across multiple DEXes in one go.
* High throughput & low fees → makes multi-pool swaps feasible.
* Aggregators become natural entry points for efficiency and simplicity.

---

While EVM cannot do such multi-DEX swaps because of on-chain gas costs.
