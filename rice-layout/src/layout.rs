//! Layout directions

use crate::{AlignmentH, AlignmentV, Div, Rect};

/// Layout directions for content
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Horizontal(AlignmentH),
    Vertical(AlignmentV),
}

impl Default for Layout {
    fn default() -> Self {
        Layout::Vertical(AlignmentV::Left)
    }
}

impl Layout {
    /// Positions pass:
    /// Compute the remaining space within a parent when removing that taken by the children.
    pub fn remaining_space(
        &self,
        node: usize,
        div: &Div,
        nodes: &[Rect],
        children: &[Vec<usize>],
    ) -> i32 {
        match self {
            Layout::Horizontal(_) => {
                let mut remaining = nodes[node].width - div.padding.left - div.padding.right;
                for (&node, div) in children[node].iter().zip(div.children.iter()) {
                    remaining -= nodes[node].width + div.margin.left + div.margin.right;
                }
                remaining
            }
            Layout::Vertical(_) => {
                let mut remaining = nodes[node].height - div.padding.bottom - div.padding.top;
                for (&node, div) in children[node].iter().zip(div.children.iter()) {
                    remaining -= nodes[node].height + div.margin.bottom + div.margin.top;
                }
                remaining
            }
        }
    }
}
