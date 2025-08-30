//! Fixed margins & padding

/// Margin & padding insets
/// - start: top if vertical, left if horizontal
/// - end: bottom if vertical, right if horizontal
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Insets {
    pub start: [i32; 2],
    pub end: [i32; 2],
}

impl Insets {
    pub fn new(top: i32, bottom: i32, left: i32, right: i32) -> Self {
        Self {
            start: [left, top],
            end: [right, bottom],
        }
    }

    pub fn uniform(value: i32) -> Self {
        Self {
            start: [value; 2],
            end: [value; 2],
        }
    }
}
