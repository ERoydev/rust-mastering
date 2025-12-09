
#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;

trait F {
    fn f(&self);
}

impl F for A {
    fn f(&self) {
        println!("{:?}", self);
    }
}

impl F for B {
    fn f(&self) {
        println!("{:?}", self);
    }
}

fn static_dispatch<T: F>(t: &T) {
    t.f();
}

fn dyn_dispatch(t: &dyn F) {
    t.f();
}

fn dyn_dispatch_box(t: Box<dyn F>) {
    t.f();
}

fn main() {
    let obj_a = A;
    static_dispatch(&obj_a);

    let obj_b = B;
    static_dispatch(&obj_b);

    let input = "A";
    // Trait object
    // Value that implements a trait,
    // but its concrete type is unknown at compile time

    // Stores the trait object on the heap, owns the value(A or B), it can outlive the current scope
    // Useful for collections, when i need ownership and dynamic dispatch
    let obj: Box<dyn F> = match input {
        "A" => Box::new(A),
        "B" => Box::new(B),
        _ => panic!(),
    };

    dyn_dispatch_box(obj);
    // Is a ref, does not own the value; just borrows it for a limited scope
    // Useful when i only need to temporarily use the trait object and dont need ownership
    let obj: &dyn F = match input {
        "A" => &A,
        "B" => &B,
        _ => panic!(),
    };

    dyn_dispatch(obj);
}