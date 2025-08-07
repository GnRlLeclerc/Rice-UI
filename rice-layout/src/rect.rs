//! Rectangle module for size & position results

/// A rectangle with position and size
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub height: i32,
    pub width: i32,
}
