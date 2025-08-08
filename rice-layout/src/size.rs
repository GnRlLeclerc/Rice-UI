//! Size constraints

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    /// Fit content
    Fit,
    /// Fixed pixel size
    Fixed(i32),
    /// Expand to a fraction of available space
    Expand(f32),
}

impl Default for Size {
    fn default() -> Self {
        Size::Fit
    }
}
