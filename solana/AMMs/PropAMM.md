## Prop AMM Explanation

PropAMM improves Solana routers by making liquidity predictable and swaps cheaper to simulate, enabling more efficient, reliable, and low-slippage routing across multiple AMMs in a single transaction.

---

### What is On-chain Router
An on-chain router is a program where the user specifies the tokens to swap (and optionally the target AMM), and your program handles all interaction with that AMM via CPI. For example: Radium, Orca. It automatically selects and executes the best swap path across one or more liquidity protocols (such as Raydium or Orca) to give the user the best price, typically in a single atomic transaction.

The Router bundles all the cross-program invocations (CPIs) needed for a swap into a single transaction. Example:

```
Router
 ├─ CPI → Raydium CLMM (tick math)
 ├─ CPI → Orca Whirlpool (tick math)
 └─ CPI → Token program
```

The problem: if the combined computation exceeds Solana’s compute budget, the transaction fails completely.

---

#### Downsides
While routers improve swap pricing by aggregating liquidity across protocols, they are limited by on-chain compute constraints, often rely on off-chain route discovery, require many accounts per transaction, and can produce suboptimal execution for large trades or expose users to MEV. CLMMs make swaps computationally expensive (tick traversal, CPIs).

---

### What is CLMM
- As a liquidity provider in such system, I can deposit tokens into a pool within chosen price range, so my capital is concentrated where trading occurs.

---

### What is PropAMM and how it solves the problems
Typically makes AMM / liquidity mechanisms more predictable and routing more efficient on chain.

#### Problem 1: High compute cost for routing
- Standard CLMMs require expensive tick math to simulate a swap
- Routers need to do this for every potential route.
- Result: High compute → transaction risk exceeding Solana compute budget.

**Solution:**
- Pre-packages liquidity behavior in a way that routers can predict price movements cheaply.
- Reduces the need for full on-chain simulations.
- Saves compute units

#### Problem 2: Complex multi-protocol routing
- Traditional routers must split trades across Raydium, Orca, CLMM pools, calculating exact amounts for each.
- Large swaps risk failing or getting poor execution.
**Solution:**
- Liquidity is structured predictably, so the router can quickly calculate optimal split paths.
- This improves execution quality and reduces failed swaps.

#### Problem 3: Slippage and unpredictable price movement
- In CLMMs, price moves non-linearly depending on liquidity in ticks.
- Routers often approximate → can cause slippage or suboptimal routes.

**Solution:**
- Makes price changes more linear and predictable from the router’s perspective, enables better “best price” routing on-chain.

---

### Mental flow
User → Router → PropAMM + Other Pools → Tokens

---

**Router without PropAMM:** Like navigating a city without a GPS — you have to check every possible street yourself. Expensive, slow, sometimes you hit a dead-end.

**Router with PropAMM:** GPS included — it tells you the fastest, safest route instantly. You spend less energy and arrive at the best destination.
