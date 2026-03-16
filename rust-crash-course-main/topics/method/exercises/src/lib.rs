#[derive(Debug)]
pub struct Rectangle {
    pub top: u32,
    pub left: u32,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self{
            width, height, left:0, top:0,
        }
    }

    pub fn move_to(&mut self, top: u32, left: u32) {
        self.top = top;
        self.left = left;
    }
}
