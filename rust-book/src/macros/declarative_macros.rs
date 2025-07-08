
#[macro_export]
macro_rules! vec {
    // Pattern => code that will execute if pattern matches
    // [1, 2, 3]
    ( $( $x:expr),* ) => {
    // capture any value that match the pattern inside these parentesis ( $x:expr)
    // match 1 assing it to `$x` then match 2 assing to `$x` and then 3
        {
            // This code will be generated
            let mut temp_vec = Vec::new();
            $(
                // This will be executed for every match we get
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}