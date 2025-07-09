// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

pub struct StrSplit<'a> {
    remainder: &'a str, 
    delimiter: &'a str, 
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

// Let's me iterate -> Every iterator should implement `Item` and `next`
impl<'a> Iterator for StrSplit<'a> {
    // Rust needs to know how long it's okay to keep this pointer for (`Item`)
    // Otherwise it might use that pointer after its deallocated from memory somewhere in the code
    type Item = &'a str; // It points to the remainder
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(first_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[0..first_delim]; // slicing 
            self.remainder = &self.remainder[(first_delim + self.delimiter.len())..]; // slicing but i specify length
            Some(until_delimiter) // return
        } else if self.remainder.is_empty() {
            None
        } else {
            // if not empty just return the last part of the remainder
            let rest = self.remainder;
            self.remainder = "";
            // &'a str     &'static str
            // if i need something that lives for at least `'a` then other lifetime thats longer than `'a` can be reduced to that description
            // The opposite i cannot give a lifetime that has a shorter lifetime 

            // If you have a reference of any lifetime or a thing that contains any lifetime
            // you can assign to it anything of the same type but a longer lifetime
            Some(rest)
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();

    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();

    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}


// fn test_string_slicing_ranges() {
//     let s = "hello world";

//     assert_eq!(&s[..], "hello world");     // Full slice
//     assert_eq!(&s[..5], "hello");          // From start to 5 (exclusive)
//     assert_eq!(&s[6..], "world");          // From 6 to end
//     assert_eq!(&s[0..5], "hello");         // Explicit full range
//     assert_eq!(&s[3..8], "lo wo");         // Middle slice
// }