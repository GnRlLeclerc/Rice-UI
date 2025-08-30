/// Layout constraints
use crate::{Align, Direction, Gap, Insets, Size};

/// Layout rules
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Layout {
    pub size: [Size; 2],
    pub min_size: [Option<i32>; 2],
    pub max_size: [Option<i32>; 2],

    // Margin & padding expressed along x, y axes directions
    pub margin: Insets,
    pub padding: Insets,

    pub direction: Direction,
    pub gap: Gap,
}

impl Layout {
    pub fn new(width: Size, height: Size) -> Self {
        Self {
            size: [width, height],
            ..Default::default()
        }
    }

    pub fn margin(mut self, insets: Insets) -> Self {
        self.margin = insets;
        self
    }

    pub fn padding(mut self, insets: Insets) -> Self {
        self.padding = insets;
        self
    }

    pub fn vertical(mut self, align: Align) -> Self {
        self.direction = Direction::Vertical(align);
        self
    }

    pub fn horizontal(mut self, align: Align) -> Self {
        self.direction = Direction::Horizontal(align);
        self
    }

    pub fn gap(mut self, gap: Gap) -> Self {
        self.gap = gap;
        self
    }

    pub fn max_width(mut self, max: i32) -> Self {
        self.max_size[0] = Some(max);
        self
    }

    pub fn max_height(mut self, max: i32) -> Self {
        self.max_size[1] = Some(max);
        self
    }

    pub fn min_width(mut self, min: i32) -> Self {
        self.min_size[0] = Some(min);
        self
    }

    pub fn min_height(mut self, min: i32) -> Self {
        self.min_size[1] = Some(min);
        self
    }

    pub fn clip_size(&self, dim: usize, size: i32) -> i32 {
        let mut clipped = size;
        if let Some(min) = self.min_size[dim] {
            clipped = clipped.max(min);
        }
        if let Some(max) = self.max_size[dim] {
            clipped = clipped.min(max);
        }
        clipped
    }
}
