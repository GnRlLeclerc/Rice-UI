//! Margin & padding

/// Fixed pixel insets
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Insets {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Default for Insets {
    fn default() -> Self {
        Insets {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }
}

impl Insets {
    /// Create vertical insets
    pub fn vertical(top: i32, bottom: i32) -> Self {
        Insets {
            left: 0,
            top,
            right: 0,
            bottom,
        }
    }

    /// Create horizontal insets
    pub fn horizontal(left: i32, right: i32) -> Self {
        Insets {
            left,
            top: 0,
            right,
            bottom: 0,
        }
    }

    /// Create all insets with the same value
    pub fn uniform(value: i32) -> Self {
        Insets {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }
}
