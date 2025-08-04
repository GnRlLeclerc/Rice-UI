//! Rectangle module for size & position results

/// A rectangle with position and size
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub height: usize,
    pub width: usize,
}
