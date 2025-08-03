//! Layout computation

use slotmap::{DefaultKey, SlotMap};

use crate::{div::Div, rect::Rect};

pub struct Arena {
    /// Rect arena for layout nodes
    pub nodes: SlotMap<DefaultKey, Rect>,
}

impl Arena {
    /// Create a new empty arena
    pub fn new() -> Self {
        Arena {
            nodes: SlotMap::with_key(),
        }
    }

    /// Compute the layout for a given root node
    pub fn compute(&mut self, root: &Div) {
        // TODO
    }
}
