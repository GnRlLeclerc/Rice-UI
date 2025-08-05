//! Layout directions

use crate::{AlignmentH, AlignmentV};

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
