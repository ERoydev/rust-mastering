
# Solana low-level task

# Task:

You are given the program with programId: `9eX2VoYXNnKbaS2HmhjgsLMpETTnvL9Tgjg2Wt6rGR1e` 

It’s deployed on Solana Devnet. 

Your goal is to Interact with the program and get the following log:

`Hehe, you've won!` 

# Solution:

To prove that you’ve solved the task provide a github repo with the Solana program client and payload that you’ve used.

Good luck :)


# My Notes for solving
A. Entrypoint is `process_instruction` which receives 3 arguments:
- program_id
- accounts
- instruction_data -> &[u8] serialized data payload 

Goal: identify this process instruction function within the disassembled/decompiled code

B: Instruction Dispatch using instruction DISCRIMINATOR:
- Simple Byte Dispatcher using Rust match => 0 -> (initialize), 1 -> (create) ...
- Anchor DISCRIMINATOR => `sha256("global:initialize")` → 8-byte discriminator, which stands at the beginning of each instruction serialized data. (Final instruction data is `[ discriminator | serialized_args ]` )

Goal: I must identify the dispatcher logic (native program or anchor program) essential to map out diff functions (instruction handlers) the program exposes

D: All accounts are passed bundled together in &[AccountInfo] slice. Inside programs, specific accounts are accessed by their numerical index within this slice (accounts[0], accounts[1], accounts[2], etc.)
- The raw bytecode only shows indexed access like accounts[2].data or checks like accounts[1].is_signer. 
- Figuring out that accounts[1] is expected to be the `user_authority` signer and accounts[2] must be the program_state PDA owned by the current program is manual process of analysis. 
- Understand the validation checks (is_signer, is_writer, owner checks, PDA verification via `invoke_signed`)

Goal: Determine, for each instruction handler, which real-world account corresponds to each index used in the code.

Before trying to decipher the actual compiled program bytecode, first understand.
- BPF base
- the single entry point
- the instruction dispatch mechanism
- indexed account array


# How i solved it

1. I used the tools first to download the .so eBPF `solana program dump <program_id>`
2. Then i used the platform-tools like `llvm-objdump`:
   - `llvm-objdump --demangle --disassemble-all dumped.so > disassemble.txt`
3. Assumptions:
   - Looking at the disassembly i found four places where `r1` register is used in a similar way, which is the argument register.
   - I see that the code have some conditions but overall it should accept 4 arguments in the instruction data that are `u64` and i just tried sending that as an instruction data.
   - By chance, at the start i expected the program to require an account, but it turns out this program does not expect any accounts.


# Explanations

`r2 = *(u64 *)(r1 + 0x10)`

- `r1` - is a pointer, in Solana, at the entrypoint r1 points to the start of the `instruction_data` buffer
- `r1 + 0x10` - means start by (16 byte in decimal) and read 8 bytes starting at that offset in our example.
- `*(u64 *)(r1 + 0x10)` - means treat the memory at that location as a 64-bit little-endian integer and read it.
- `r2 = ` means store that 64-bit value into register `r2`

```sh
r2 = *(u64 *)(r1 + 0x0)
r2 = *(u64 *)(r1 + 0x8)
r2 = *(u64 *)(r1 + 0x10)
r1 = *(u64 *)(r1 + 0x18)
```