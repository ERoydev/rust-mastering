use std::ops::Deref;


// My custom smart pointer
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// With This i implement Deref `Trait` allowing MyBox to use dereference operator
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn deref_trait() {
    let x = 5;
    let y1 = &x; // Reference to x
    let y = Box::new(x); // Use box like reference

    // Box pointing to a value stored in memory just like `reference operator`, so i can use deference operator in the same way

    assert_eq!(5, x);
    assert_eq!(5, *y);  
    // The only difference is that `y` is pointing to a `Copy` of `y`

    // Because when primitives such as integers get passed to a function:
    //  - the value is copied instead of ownership being transfered

    // The Box<T> is a smart pointer:
    //  - that implements the deref trait which allows deref operator to work the same as if it were a reference



    // ===== Deref Trait Example ===============================================
    let y2 = MyBox::new(x);
    assert_eq!(5, *y2); // If i do not implement Deref Trait this will return error
    // When i use this `*y2` -> Rust actually calls *(y2.deref()) -> to ge the reference and the we use dereference operator



    // ===== Deref Coercions ===================================================
    // Converts a reference from one type to a reference from different type

    let m: MyBox<String> = MyBox::new(String::from("Rust"));
    hello(&m); // Rust see that the type passed of `m` is different, so it runs `chained deref calls to get the correct type` !Important, Explained Bellow

    // &MyBox<String> -> When i deref i receive -> &String -> When i deref this i get -> &str
    // &MyBox<String> -> &String -> &str => Because they all have the Deref Trait from rust library implemented (&String and MyBox which i implement it Manually)

    hello(&(m)[..]); // Without automatic `deref coercion` i will write it like this !

    // I could implement DerefMut => for mutable references

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    // Deref coercion is Rust’s automatic way to convert references of smart pointers to references of inner types.



    // ======= Exercises


}

// ==================== REAL WORLD EXAMPLE OF USING DEREF TRAIT
// use std::ops::Deref;

// A simple configuration struct with some data and a method
struct Config {
    url: String,
    port: u16,
}

impl Config {
    // Method to return the full address as a string
    fn address(&self) -> String {
        format!("{}:{}", self.url, self.port)
    }
}

// Custom smart pointer-like wrapper around Config
struct MyConfigBox(Config);

impl MyConfigBox {
    // Constructor for MyConfigBox
    fn new(config: Config) -> Self {
        MyConfigBox(config)
    }
}

// Implement Deref to allow MyConfigBox to behave like &Config
impl Deref for MyConfigBox {
    type Target = Config;

    // This method returns a reference to the inner Config instance
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    // Create a Config instance
    let config = Config {
        url: "localhost".to_string(),
        port: 8080,
    };

    // Wrap it in MyConfigBox
    let my_config = MyConfigBox::new(config);

    // Now, why does this work? `my_config.address()` works because of Rust's *deref coercion*:
    //
    // 1. `MyConfigBox` does NOT have an `address` method itself.
    // 2. Rust sees the method call and looks if it can dereference `my_config` to find `address`.
    // 3. Because we implemented `Deref<Target=Config>`, Rust calls `deref` and gets `&Config`.
    // 4. `Config` *has* an `address` method, so Rust calls it.
    //
    // This automatic conversion from `&MyConfigBox` to `&Config` via the `deref()` method
    // is called *deref coercion*, and it makes using wrapper types very ergonomic.
    //
    // Without `Deref`, you would have to manually dereference like this:
    // `(*my_config).address()`
    println!("Server address: {}", my_config.address());
}
