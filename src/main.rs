use trait_objects::{Button, Draw, Screen};

struct SelectBox {
    _width: u32,
    _hieght: u32,
    _options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        // draw select box
        println!("Drawing select box");
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                _width: 100,
                _hieght: 100,
                _options: vec![
                    String::from("yes"),
                    String::from("maybe"),
                    String::from("no")
                ]
            }),
            Box::new(Button {
                width: 60,
                height: 60,
                label: String::from("Button")
            })
        ]
    };

    screen.run();
}
