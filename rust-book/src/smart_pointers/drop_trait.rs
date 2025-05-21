

// Tipically handled automatically but can be customized when i use .lock() values to clean 
struct CustomSmartPointer {
    data: String,
} 

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn drop_trait() {
    let c = CustomSmartPointer{
        data: String::from("my stuff"),
    };
}