# Deref trait implementation                                                                                                                                                             
- Resource: https://oneuptime.com/blog/post/2026-01-25-auto-dereferencing-rules-rust/view
- At the rust-atomics-and-locks i have good real world example -> [here](https://github.com/ERoydev/rust-mastering/blob/main/rust-atomics-and-locks/building-own-arc/src/basic_ref_counting.rs#L75-L90)
   
  ## Auto-deref                                                                                                                                                                                                                         
                                                                  
  **When does it fire?** Only through the `.` operator (method calls and field access).                                                                                                                                                 
  It does NOT fire for: `==`, `+`, `<`, function arguments, or pattern matching.
                                                                                                                                                                                                                                        
  **The chain (memorize this):**                                                                                                                                                                                                        
  `T → &T → &mut T → *T → &T' → &mut T' → ...`                                                                                                                                                                                          
                                                                                                                                                                                                                                        
  Code example:                                                                                                                                                                                                                         
   
  ```rust                                                                                                                                                                                                                               
  struct Wrapper<T>(T);                                           

  impl<T> Deref for Wrapper<T> {
      type Target = T;
      fn deref(&self) -> &T {
          &self.0                                                                                                                                                                                                                       
      }
  }                                                                                                                                                                                                                                     
                                                                  
  struct Inner;
  impl Inner {
      fn hello(&self) -> &'static str {
          "hi from Inner"                                                                                                                                                                                                               
      }
  }                                                                                                                                                                                                                                     
                                                                  
  // Then in main:
  let w = Wrapper(Inner);
  println!("1: {}", w.hello());                                                                                                                                                                                                         
  ```
                                                                                                                                                                                                                                        
  `w.hello()` looks strange — like `Wrapper` inherits `.hello()` from the inner type. Here's the mental model:                                                                                                                          
   
  ```                                                                                                                                                                                                                                   
  Level 0:  Wrapper<Inner>     ← has .hello()?  No                
            &Wrapper<Inner>    ← has .hello()?  No                                                                                                                                                                                      
            &mut Wrapper       ← has .hello()?  No
            ↓ apply * (Deref)                                                                                                                                                                                                           
  Level 1:  Inner              ← has .hello()?  No (hello takes &self)                                                                                                                                                                  
            &Inner             ← has .hello()?  ✅ YES — stop.
  ```                                                                                                                                                                                                                                   
                                                                  
  At every level the compiler tries **all three forms** (`T`, `&T`, `&mut T`) **before** descending via `Deref`.                                                                                                                        
   
  ### High-level intuition (less precise — glosses over the auto-ref step)                                                                                                                                                              
  - Compiler asks: does `Wrapper<Inner>` have `.hello()`? **No**. 
  - Does `Wrapper<Inner>` implement `Deref`? **Yes**.                                                                                                                                                                                   
  - Deref produces `Inner`; auto-ref makes it `&Inner`. Does `&Inner` have `.hello()`? **Yes** ✓.                                                                                                                                                                                          
   
  ## Auto-deref vs deref coercion (don't confuse them)                                                                                                                                                                                  
                                                                  
  Same trait (`Deref`), different rules:                                                                                                                                                                                                
                                                                  
  | | Auto-deref | Deref coercion |                                                                                                                                                                                                     
  |---|---|---|                                                   
  | Trigger | `.` operator | type boundary (fn arg, assignment, return) |
  | Goal | find a method/field | satisfy an expected reference type |                                                                                                                                                                   
  | Example | `w.hello()` | `takes_str(&owned)` where `owned: String` |                                                                                                                                                                 
                                                                                                                                                                                                                                        
  Neither fires for operators (`==`, `+`, `<`) — that's why `*a == b` needs the explicit `*`.                                                                                                                                           
                                                                                                                                                                                                                                        
  ## Summary                                                                                                                                                                                                                            
                                                                  
  Auto-deref makes smart pointers and wrappers feel transparent — but only through the **dot operator**, and only when **no inherent method on the outer type matches first**.    
