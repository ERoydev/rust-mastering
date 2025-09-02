# What is a Buffer?

A buffer is simply a list (array) of bytes that holds your raw binary data.

## In Serialization
- All data (numbers, strings, arrays) is converted to bytes and stored in the buffer.
- You read from the buffer to interpret the data according to your struct layout.

## Example
If you serialize a struct:
- The buffer might look like: `[69, 109, 105, 108, 0, 24, 4, 0, 0, 0, 82, 117, 115, 101]`
- Each group of bytes represents a field (e.g., name, age, location).

## Little-endian
- Multi-byte numbers (u16, u32, u64, etc.) are stored in the buffer in little-endian order (least significant byte first).
- When reading from the buffer, you interpret these bytes as little-endian numbers.

## Summary
- Buffer = list of bytes for your raw binary data.
- All serialization/deserialization happens by reading/writing bytes in the buffer.
- Little-endian affects how multi-byte numbers are stored and read from the buffer.
