//! A simple div element

use crate::{AlignmentH, AlignmentV, Gap, Insets, layout::Layout, size::Size};

#[derive(Debug, Clone, Default)]
pub struct Div {
    /// Width constraint
    pub width: Size,
    /// Optional minimum width
    pub min_width: Option<i32>,
    /// Optional maximum width
    pub max_width: Option<i32>,
    /// Height constraint
    pub height: Size,
    /// Optional minimum height
    pub min_height: Option<i32>,
    /// Optional maximum height
    pub max_height: Option<i32>,
    /// Margin insets
    pub margin: Insets,
    /// Padding insets
    pub padding: Insets,
    /// Layout for the children
    pub layout: Layout,
    /// Gap between children
    pub gap: Gap,
    /// Children elements
    pub children: Vec<Div>,
}

impl Div {
    /// Create a new div with specified width and height
    pub fn new(width: Size, height: Size) -> Self {
        Div {
            width,
            min_width: None,
            max_width: None,
            height,
            min_height: None,
            max_height: None,
            margin: Insets::default(),
            padding: Insets::default(),
            layout: Layout::default(),
            gap: Gap::default(),
            children: vec![],
        }
    }

    /// Set the width of the div
    pub fn width(mut self, width: Size) -> Self {
        self.width = width;
        self
    }

    /// Set the height of the div
    pub fn height(mut self, height: Size) -> Self {
        self.height = height;
        self
    }

    /// Set the children of the div
    pub fn children(mut self, children: Vec<Div>) -> Self {
        self.children = children;
        self
    }

    /// Set the layout of the div
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    /// Set layout to vertical
    pub fn vertical(mut self, align: AlignmentV) -> Self {
        self.layout = Layout::Vertical(align);
        self
    }

    /// Set layout to horizontal
    pub fn horizontal(mut self, align: AlignmentH) -> Self {
        self.layout = Layout::Horizontal(align);
        self
    }

    /// Set the margin insets
    pub fn margin(mut self, margin: Insets) -> Self {
        self.margin = margin;
        self
    }

    /// Set the padding insets
    pub fn padding(mut self, padding: Insets) -> Self {
        self.padding = padding;
        self
    }

    /// Set the gap between children
    pub fn gap(mut self, gap: Gap) -> Self {
        self.gap = gap;
        self
    }

    /// Set the minimum width
    pub fn min_width(mut self, min_width: i32) -> Self {
        self.min_width = Some(min_width);
        self
    }

    /// Set the maximum width
    pub fn max_width(mut self, max_width: i32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    /// Set the minimum height
    pub fn min_height(mut self, min_height: i32) -> Self {
        self.min_height = Some(min_height);
        self
    }

    /// Set the maximum height
    pub fn max_height(mut self, max_height: i32) -> Self {
        self.max_height = Some(max_height);
        self
    }

    // ************************************************* //
    //                     UTILITIES                     //
    // ************************************************* //

    pub(crate) fn clip_height(&self, height: i32) -> i32 {
        if let Some(max_height) = self.max_height {
            height.min(max_height)
        } else if let Some(min_height) = self.min_height {
            height.max(min_height)
        } else {
            height
        }
    }

    pub(crate) fn clip_width(&self, width: i32) -> i32 {
        if let Some(max_width) = self.max_width {
            width.min(max_width)
        } else if let Some(min_width) = self.min_width {
            width.max(min_width)
        } else {
            width
        }
    }
}
}
}
