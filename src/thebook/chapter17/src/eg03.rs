// eg17-3
pub trait Draw {
    fn draw(&self);
}

// eg17-4
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// eg17-5
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// ================== copy from eg06.rs ==================
// eg17-7
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

// eg17-8
// use gui::Draw;

pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}