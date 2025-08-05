//! A simple div element

use crate::{Insets, layout::Layout, size::Size};

#[derive(Debug, Clone)]
pub struct Div {
    /// Width constraint
    pub width: Size,
    /// Height constraint
    pub height: Size,
    /// Margin insets
    pub margin: Insets,
    /// Padding insets
    pub padding: Insets,
    /// Layout for the children
    pub layout: Layout,
    /// Children elements
    pub children: Vec<Div>,
}

impl Default for Div {
    fn default() -> Self {
        Div {
            width: Size::Fit,
            height: Size::Fit,
            margin: Insets::default(),
            padding: Insets::default(),
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
            margin: Insets::default(),
            padding: Insets::default(),
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

    /// Set layout to vertical
    pub fn vertical(mut self) -> Self {
        self.layout = Layout::Vertical;
        self
    }

    /// Set layout to horizontal
    pub fn horizontal(mut self) -> Self {
        self.layout = Layout::Horizontal;
        self
    }

    /// Set the margin insets
    pub fn with_margin(mut self, margin: Insets) -> Self {
        self.margin = margin;
        self
    }

    /// Set the padding insets
    pub fn with_padding(mut self, padding: Insets) -> Self {
        self.padding = padding;
        self
    }
}
