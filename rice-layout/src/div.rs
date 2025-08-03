//! A simple div element

use crate::size::Size;

#[derive(Debug, Clone)]
pub struct Div {
    /// Width constraint
    pub width: Size,
    /// Height constraint
    pub height: Size,
}

impl Default for Div {
    fn default() -> Self {
        Div {
            width: Size::Fit,
            height: Size::Fit,
        }
    }
}
