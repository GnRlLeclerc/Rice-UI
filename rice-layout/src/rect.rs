//! Computed size and positions for a layout element

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Rect {
    pub size: [i32; 2],
    pub position: [i32; 2],
}

impl Rect {
    pub fn width(&self) -> i32 {
        self.size[0]
    }

    pub fn height(&self) -> i32 {
        self.size[1]
    }

    pub fn x(&self) -> i32 {
        self.position[0]
    }

    pub fn y(&self) -> i32 {
        self.position[1]
    }
}
