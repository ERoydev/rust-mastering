use std::ops::{Shl, Shr};


struct ParsedInstruction {
    class: u8,
    opcode: u8,
    dst_reg: u8,
    src_reg: u8,
    offset: u16,
    immediate: u32,
}

impl ParsedInstruction {
    pub fn from_bytes(instruction: &[&str]) -> Option<Self> {
        if instruction.len() < 8 { return None; }
        let bytes: Vec<u8> = instruction.iter()
            .map(|b| u8::from_str_radix(b, 16).ok())
            .collect::<Option<Vec<u8>>>()?;

        // Parse the instruction string as a hexadecimal byte
        // index 0: class and opcode
        let class = 0b00000111 & bytes[0];
        let opcode = (bytes[0] >> 3) & 0b00011111;

        // index 1: dst_reg and src_reg
        let dst_reg = 0b00001111 & bytes[1];
        let src_reg = (bytes[1] >> 4) & 0b00001111;

        // Combine byte index 2 and index 3
        let high = bytes[2];
        let low = bytes[3];
        // By casting high to `u16` i extend it to 16 bits, and then when i shift it left by 8 bits
        // it moves its value to the most significant bit position in order to make room for the low part,
        // result in combination of the both bytes into 16-bit value
        let offset = ((high as u16) << 8) | (low as u16);

        // I have to combine index from 4 to 7
        let first_byte = bytes[4];
        let second_byte = bytes[5];
        let third_byte = bytes[6];
        let fouth_byte = bytes[7];
        let immediate = (first_byte as u32) << 24
            | (second_byte as u32) << 16
            | (third_byte as u32) << 8
            | (fouth_byte as u32);

        // fouth_byte.shr(rhs);
        // first_byte.shl(rhs);
        // wrap();
        // saturating();
        Some(Self { class, opcode, dst_reg, src_reg, offset, immediate })
    }
}

const EBPF: &str = "
    36:	18 00 00 00 00 00 00 00 00 00 00 00 03 00 00 00	r0 = 0x300000000 ll
    38:	79 12 00 00 00 00 00 00	r2 = *(u64 *)(r1 + 0x0)
    39:	55 02 15 00 00 00 00 00	if r2 != 0x0 goto +0x15 <entrypoint+0xc8>
    40:	79 12 08 00 00 00 00 00	r2 = *(u64 *)(r1 + 0x8)
    41:	55 02 0e 00 10 00 00 00	if r2 != 0x10 goto +0xe <entrypoint+0xa0>
    42:	79 12 10 00 00 00 00 00	r2 = *(u64 *)(r1 + 0x10)
    43:	18 03 00 00 00 00 00 00 00 00 00 00 00 ff ff ff	r3 = -0x10000000000 ll
    45:	5f 32 00 00 00 00 00 00	r2 &= r3
    46:	18 03 00 00 00 00 00 00 00 00 00 00 00 9b 6a d6	r3 = -0x2995650000000000 ll
    48:	af 32 00 00 00 00 00 00	r2 ^= r3
    49:	79 11 18 00 00 00 00 00	r1 = *(u64 *)(r1 + 0x18)
    50:	4f 12 00 00 00 00 00 00	r2 |= r1
    51:	55 02 04 00 00 00 00 00	if r2 != 0x0 goto +0x4 <entrypoint+0xa0>
    52:	18 01 00 00 f0 01 00 00 00 00 00 00 00 00 00 00	r1 = 0x1f0 ll
    54:	b7 02 00 00 11 00 00 00	r2 = 0x11
    55:	05 00 03 00 00 00 00 00	goto +0x3 <entrypoint+0xb8>
    56:	18 01 00 00 01 02 00 00 00 00 00 00 00 00 00 00	r1 = 0x201 ll
    58:	b7 02 00 00 22 00 00 00	r2 = 0x22
    59:	85 10 00 00 ff ff ff ff	call -0x1
    60:	b7 00 00 00 00 00 00 00	r0 = 0x0
    61:	95 00 00 00 00 00 00 00	exit
";

fn main() {
    let ebpf_lines: Vec<&str> = EBPF.lines().collect();

    for line in ebpf_lines {
        let vector_of_line: Vec<&str> = line.split("\t").collect();
        if vector_of_line.len() < 2 {
            continue;
        }

        let _instruction_offset = vector_of_line[1];
        let ebpf_instruction: Vec<&str> = vector_of_line[1].split_whitespace().collect();

        let parsed = ParsedInstruction::from_bytes(&ebpf_instruction).unwrap();

        println!("Prased: {}", parsed.opcode);   
    }
}
