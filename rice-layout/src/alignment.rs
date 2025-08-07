//! Alignment along a layout

use crate::{Div, Rect};

/// Alignment for horizontal layouts
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AlignmentH {
    Top,
    Center,
    Bottom,
}

/// Alignment for vertical layouts
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AlignmentV {
    Left,
    Center,
    Right,
}

impl AlignmentV {
    /// Compute the x position of a child rectangle within a parent rectangle given the alignment
    pub fn position_x(
        &self,
        parent_rect: &Rect,
        child_rect: &Rect,
        parent_div: &Div,
        child_div: &Div,
    ) -> i32 {
        match self {
            // Stick to left: only parent padding & child margin + border
            AlignmentV::Left => {
                parent_rect.x
                    + parent_div.padding.left
                    + child_div.margin.left
                    + child_div.border.left
            }
            // Center: center child according to its width + margin within parent padding
            AlignmentV::Center => {
                parent_rect.x
                    + parent_div.padding.left
                    + (parent_rect.width
                        - parent_div.padding.right
                        - parent_div.padding.left
                        - child_rect.width
                        - child_div.margin.left
                        - child_div.border.left
                        - child_div.margin.right
                        - child_div.border.right)
                        / 2
            }
            // Stick to right: only parent padding & child margin + border
            AlignmentV::Right => {
                parent_rect.x + parent_rect.width
                    - parent_div.padding.right
                    - child_div.margin.right
                    - child_div.border.right
                    - child_rect.width
            }
        }
    }
}

impl AlignmentH {
    /// Compute the y position of a child rectangle within a parent rectangle given the alignment
    pub fn position_y(
        &self,
        parent_rect: &Rect,
        child_rect: &Rect,
        parent_div: &Div,
        child_div: &Div,
    ) -> i32 {
        match self {
            // Stick to top: only parent padding & child margin + border
            AlignmentH::Top => {
                parent_rect.y + parent_div.padding.top + child_div.margin.top + child_div.border.top
            }
            // Center: center child according to its height + margin within parent padding
            AlignmentH::Center => {
                parent_rect.y
                    + parent_div.padding.top
                    + (parent_rect.height
                        - parent_div.padding.bottom
                        - parent_div.padding.top
                        - child_rect.height
                        - child_div.margin.top
                        - child_div.border.top
                        - child_div.margin.bottom
                        - child_div.border.bottom)
                        / 2
            }
            // Stick to bottom: only parent padding & child margin + border
            AlignmentH::Bottom => {
                parent_rect.y + parent_rect.height
                    - parent_div.padding.bottom
                    - child_div.margin.bottom
                    - child_div.border.bottom
                    - child_rect.height
            }
        }
    }
}
