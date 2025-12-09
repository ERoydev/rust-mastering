
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // TODO: Learn why it is good to use Smart pointers when defining Trait as type

}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // draw button
    }
}

// Generic implementation
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
//     // Problem: I am limited to only one

// }

// impl<T> Screen<T> 
// where T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }