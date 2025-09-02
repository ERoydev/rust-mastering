
# Borsh & Anchor Serialization Cheat Sheet

## Key Concepts
- **Little-endian**: Multi-byte numbers (u16, u32, u64, etc.) are encoded with the least significant byte first.
- **Strings**: Encoded as a 4-byte little-endian length prefix, followed by UTF-8 bytes.
- **Fixed-size arrays**: No length prefix; just the bytes, padded if needed.
- **Single-byte values**: (u8, i8, ASCII chars) are stored directly, no endianness.

---

## Example Struct
- Lets say for example we have this struct 

```rust
MyStruct {
    #[max_len(5)] // fixed-size, no prefix
    name = "Emil",
    age = 24,
    location = "Ruse",
}
```

### Serialization Layout
| Field     | Bytes (hex)             | Bytes (decimal)                | Meaning                |
|-----------|-------------------------|--------------------------------|------------------------|
| name      | 45 6D 69 6C 00          | 69 109 105 108 0               | "Emil" + padding      |
| age       | 18                      | 24                             | age 24                 |
| location  | 04 00 00 00 52 75 73 65 | 4 0 0 0 82 117 115 101         | length 4, "Ruse"       |

### Full Byte Array
```
[69, 109, 105, 108, 0, 24, 4, 0, 0, 0, 82, 117, 115, 101]
```

---

## How to Decode
1. Read 5 bytes for `name` (fixed-size): "Emil" + padding.
2. Read 1 byte for `age` (u8): 24.
3. Read 4 bytes for `location` length (little-endian u32): 4. Since its prefixed first 4 bytes specify the length of that field.
4. Read next 4 bytes for `location` string: "Ruse".

---

## General Rules
- For multi-byte numbers, always interpret bytes as little-endian.
- For strings, always expect a length prefix (unless fixed-size).
- For fixed-size arrays, just read the bytes in order.
- For single bytes, just read the value.

---

## Not Cryptography
This is binary serialization (data layout), not encryption or security.

---

## Quick Reference
- Little-endian: LSB first for numbers.
- Strings: `[length (u32 LE)] [UTF-8 bytes]`
- Fixed-size: `[bytes]`
- Example: "apple" → `05 00 00 00 61 70 70 6C 65`

---

**Use this guide to reason about byte layouts for Borsh/Anchor accounts and structs!**
