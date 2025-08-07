//! Size constraints

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Size {
    /// Fixed pixel size
    Fixed(i32),
    /// Fit content
    Fit,
}

impl Default for Size {
    fn default() -> Self {
        Size::Fit
    }
}
