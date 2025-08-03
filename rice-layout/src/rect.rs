//! Rectangle module for size & position results

use slotmap::DefaultKey;

/// A rectangle with position and size
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub height: usize,
    pub width: usize,

    /// Children rectangle keys in the arena
    pub children: Vec<DefaultKey>,
}
