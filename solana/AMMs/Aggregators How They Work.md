
## Aggregators how they work

On Solana, aggregators like Jupiter query multiple AMMs off-chain, calculate the best route considering price and compute cost, build a single atomic transaction, and execute complex multi-pool swaps seamlessly for the user.

- Are like traffic controllers for the swap, coordinate liquidity from multiple sources (AMMs, proprietary pools), also optimize for Solana’s compute constraints

---

### Flow:
1. **User → Swap 10 SOL → USDC** → Aggregator like Jupiter receives this request.
2. **Downstream Prompting**
    - The aggregator off-chain engine queries multiple AMMs for current rates.
    - Sources like Radium, Orca (public pools), Proprietary AMMs.
    - The aggregator collects the quotes almost instantly.

3. **Optimal Route Calculation**
    - The aggregator calculates the best path using: Price quote from each AMM, CU cost for each possible route, oracle freshness (avoid stale prices).
    - The result is single optimized route, which may involve: Splitting the swap across multiple pools (Radium 60%, Orca 40%) or Multi-hop swaps (SOL → mSOL → USDC) if it improves the final price.
    - This is where compute efficiency and predictive AMMs like PropAMM really matter — they allow the aggregator to calculate routes without exceeding Solana’s compute budget.

4. **Transaction Composition**
    - The aggregator constructs one atomic Solana transaction containing all necessary instructions.
    - This includes all CPIs to each pool involved in the swap.
    - Thanks to SVM atomicity, either the entire swap executes, or it fails — no partial fills.

5. **User Execution:**
    - The user signs one transaction. The trade executes across multiple AMMs in a single step, seamlessly from the user’s perspective.

---

### Key takeaways
1. Aggregators do off-chain heavy lifting to reduce on-chain compute usage.
2. They optimize swaps not just for price, but also for CU cost and oracle freshness.
3. Multi-pool and multi-hop swaps are transparent to the user — they see one simple interface.
4. PropAMM or other proprietary AMMs improve aggregator efficiency by providing predictable liquidity and price outputs, reducing computation required for routing.

