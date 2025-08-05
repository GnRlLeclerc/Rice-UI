//! Margin & padding

/// Fixed pixel insets
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Insets {
    pub left: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
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
    pub fn vertical(top: usize, bottom: usize) -> Self {
        Insets {
            left: 0,
            top,
            right: 0,
            bottom,
        }
    }

    /// Create horizontal insets
    pub fn horizontal(left: usize, right: usize) -> Self {
        Insets {
            left,
            top: 0,
            right,
            bottom: 0,
        }
    }

    /// Create all insets with the same value
    pub fn uniform(value: usize) -> Self {
        Insets {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }
}
