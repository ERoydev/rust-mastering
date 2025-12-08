# Rust `std::mem` Module Overview

The `std::mem` module in Rust provides functions for working with memory and manipulating values at a low level. It is commonly used for operations that involve moving, replacing, or swapping values, as well as inspecting type sizes and alignments.

## Common Functions

### 1. `mem::swap`
Swaps the values of two mutable references.

It is efficient and can be used even if types doesn't implement `Copy` or `Clone`
It directly swaps the values in memory and handling ownership and borrowing rules correctly. 
**Avoids unnecessary clones or moves!**
```rust
use std::mem;
let mut a = 1;
let mut b = 2;
mem::swap(&mut a, &mut b); // a = 2, b = 1
```

### 2. `mem::replace`
Replaces the value at a mutable reference with a new value, returning the old value.
```rust
let mut x = String::from("hello");
let old = std::mem::replace(&mut x, String::from("world"));
// x = "world", old = "hello"
```

### 3. `mem::take`
Replaces the value at a mutable reference with its default value, returning the old value.
```rust
let mut s = Some(42);
let old = std::mem::take(&mut s);
// s = None, old = Some(42)
```

### 4. `mem::size_of` and `mem::align_of`
Returns the size (in bytes) and alignment of a type.
```rust
let size = std::mem::size_of::<u32>(); // 4
let align = std::mem::align_of::<u32>(); // 4
```

## Use Cases
- Swapping or replacing values efficiently
- Taking ownership of a value while leaving a default behind
- Inspecting type layout for FFI or optimization

## Safety
Some functions in `std::mem` are unsafe and should be used with care. Always consult the official documentation for details.

## My example

```rust
fn main() {
    // std::mem
    // swap - switch the value between two items
    // take - take the value and leave default behind
    // replace - take the value out, leave behind the thing you tell it to leave
    
    let mut one_ring = Ring {
        owner: "Bilbo Baggings".to_string(),
        former_owner: "Gollum".to_string()
    };

    println!("{:?}", one_ring);

    /*
    Instead of using temporary value to swap:
        - Requires cloning or moving the values which is inefficient (even problematic if types doesn't impl Copy or Clone)   
    
    I can avoid unnecessary allocations.
    
    mem::swap - works for any type, does not require cloning and is guaranteed to be safe and efficient
        - it directly swaps the values in memory and handling ownership and borrowing rules correctly
        - It is more efficient, especially for non-Copy types 
        - avoids unnecessary clones or moves
    */
    mem::swap(&mut one_ring.owner, &mut one_ring.former_owner);
    println!("{:?}", one_ring);

    let gollum = mem::replace(&mut one_ring.owner, "Frodo Baggins".to_string());
    println!("The value i have replaced: {:?}", gollum);
    println!("After replaces: {:?}", one_ring);

    let (name1, name2) = (mem::take(&mut one_ring.owner), mem::take(&mut one_ring.former_owner));

    println!("One Ring after take: {:?}", one_ring);
    println!("{} {}", name1, name2);
}
```
