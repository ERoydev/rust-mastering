use traits_objectc_in_rust::*;

struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // draw select box
    }
}

fn main() {
    let button = Button { width: 12, height: 23, label: String::from("click me")};

    let styles = vec!["align-self: 0".to_string(), "flex-column".to_string()];
    let select_box = SelectBox {width: 100, height: 100, options: styles};

    let screen = Screen {
        components: vec![
            Box::new(button),
            Box::new(select_box)
        ]
    };


}
