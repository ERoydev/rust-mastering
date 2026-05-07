
# Auto-ref is like auto-deref, something that the rust compiler does automatically
                             
Auto-ref = the compiler inserts & for you when a method needs a reference but you handed it an owned value.     

```rust
let x = String::from("hi");                                                                                                                                                                                                           
x.len();       // len takes &self, but x is owned String
// Compiler silently rewrites to:                                                                                                                                                                                                     
(&x).len();                                                              
```
                                                                                                                                                                                                                                                                   
That silent & is auto-ref. Same idea for &mut (auto-mut-ref) when the method takes &mut self.                                                                                                                                         
 
Why it exists: so you can write x.method() without manually borrowing every time.        
