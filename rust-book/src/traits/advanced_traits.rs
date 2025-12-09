
// Associated types - Is a placeholder that you can add to your Trait and then methods can use that placeholder
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

// Diff between Associated Types and Generic, with associated types we can have only one concrete type per implementation
// Where is with Generics we can have multiple concrete types per implementation.

struct Counter {}

impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}

impl Iterator<u16> for Counter {
    fn next(&mut self) -> Option<u16> {
        Some(0)
    }
}

// Generics allow us to have multiple implementation of the Iterator trait on a single type substituting the generic for different concrete values

// ========

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

// Default generic type parameters => can specify a default concrete type, this allows implementors to not have to specify a concrete type unless is different from a default

// Operator overloading => Customizing the behaviour of operator, customize the semantics of operator that have associated Traits in the standard library (ops module)
// Example: I can overload the Add operator for my Point struct, by implementing the Add trait for Point

impl Add for Point { // By implementing Add i define how the `+` operator works for Point objects.
    type Output = Point;

    // overloading, so i change the add method so i can now add two `Point` instances together
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    } 
}

// Super traits
use std::fmt;

// Orphan Rule -> We can implement a trait on a type as long as the trait or the type is defined within our crate

// By wrapping the type `Vec<String>` which i dont own in a wrapper which now i own, that lets me bypass the orphan rule => This is called NewType Pattern.
struct Wrapper(Vec<String>);


impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
fn main() {
    assert_eq!(
        Point { x: 1, y: 0} + Point { x:2, y: 3},
        Point{ x: 3, y: 3}
    );
}
