# Rust Bitwise Operations (Short Guide)

## 1. Bitwise Operators

| Operator | Purpose |
|----------|--------|
| `&`      | AND → keeps bits where both are 1 |
| `|`      | OR → sets bits where either is 1 |
| `^`      | XOR → toggles bits where only one is 1 |
| `!`      | NOT → flips all bits |
| `>>`     | Right shift → moves bits down, fills left with 0 |
| `<<`     | Left shift → moves bits up, fills right with 0 |

---

## 2. Masking

- Keep only certain bits using `&` with a mask:

```rust
let byte: u8 = 0b11001101;
let mask: u8 = 0b00001111;
let lower4 = byte & mask; // 00001101
```

---

## 3. Shifting

- **Right shift (`>>`)** → move bits to LSB to read value:

```rust
let byte: u8 = 0b10110110;
let top3 = byte >> 5; // 00000101 = 5
```

- **Left shift (`<<`)** → move bits to higher positions before combining:

```rust
let opcode: u8 = 0b101;
let instr = opcode << 5; // 10100000
```

---

## 4. Shift + Mask

- For middle bits, shift them to LSB then mask:

```rust
let byte: u8 = 0b01101110;
let middle3 = (byte >> 2) & 0b00000111; // 00000011 = 3
```

---

✅ **Rule of thumb**

1. Shift right to **read/extract**.  
2. Mask to **keep only desired bits**.  
3. Shift left to **store/combine** in higher bits.


## 5. The exercises i did 


```rs
/*
        Bit Indexes:
        let byte = 0b10110110;
        // Bits:        1 0 1 1 0 1 1 0
        // Indexes:     7 6 5 4 3 2 1 0
     */

    // ======= Ex1. -> shifting =================

    // Extract the top 3 bits, most significant bits (7, 5, 6) which are the first 3 bits -> 101
    let byte = 0b10110110; 
    let solution = byte >> 5;
    // The thing i did is just shift them to the least significant position, meaning to the end -> 00000101 (0b101) = 5 in decimal
    
    // ======= Ex2. -> masking =================

    // Extract the lower 4 bits (3, 2, 1, 0) -> 1101;
    let byte2 = 0b11001101;
    let mask =  0b00001111; // Masking the bits i dont want 
    let solution2 = byte2 & mask; 
    // Solution: `and -> &` operator is doing a comparison for each bit and where mask have 0 that means false so only the bits that are true (1 == 1) will return true and will be true in the result too

    /*
    Visual representation of AND:
        // 1 1 0 0  1 1 0 1  (byte2)
    AND // 0 0 0 0  1 1 1 1  (mask)
    -----------------------
        // 0 0 0 0  1 1 0 1  (solution2)
    */


    // ======= Ex3. -> shift + masking =================
    let byte3 = 0b01101110; // Extract the middle 3 bits -> (4, 3, 2)

    let shift_to_least_significant_position = byte3 >> 2;
    println!("{:08b}", shift_to_least_significant_position);
    let mask = 0b00000111;

    let solution3 = shift_to_least_significant_position & mask;

    // I shifted the middle value to the least significant bits and then i masked the rest that i don't need
```
