//! A simple div element

use crate::{layout::Layout, size::Size};

#[derive(Debug, Clone)]
pub struct Div {
    /// Width constraint
    pub width: Size,
    /// Height constraint
    pub height: Size,
    /// Children elements
    pub children: Vec<Div>,
    /// Layout for the children
    pub layout: Layout,
}

impl Default for Div {
    fn default() -> Self {
        Div {
            width: Size::Fit,
            height: Size::Fit,
            children: vec![],
            layout: Layout::Vertical,
        }
    }
}

impl Div {
    /// Create a new div with specified width and height
    pub fn new(width: Size, height: Size) -> Self {
        Div {
            width,
            height,
            children: vec![],
            layout: Layout::Vertical,
        }
    }

    /// Set the width of the div
    pub fn with_width(mut self, width: Size) -> Self {
        self.width = width;
        self
    }

    /// Set the height of the div
    pub fn with_height(mut self, height: Size) -> Self {
        self.height = height;
        self
    }

    /// Set the children of the div
    pub fn with_children(mut self, children: Vec<Div>) -> Self {
        self.children = children;
        self
    }

    /// Set the layout of the div
    pub fn with_layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }
}
