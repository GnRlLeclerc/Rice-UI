//! Layout computation

use slotmap::{DefaultKey, SlotMap};

use crate::{div::Div, rect::Rect, size::Size};

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
    pub fn compute(&mut self, root: &Div) -> DefaultKey {
        // 1st pass: compute fixed widths (top-down)
        let root_key = self.recurse_fixed_width(root);

        // 2nd pass: compute grow widths (bottom-up)
        // TODO:

        // Return root key
        root_key
    }

    /// Remove a node from the arena by its key
    /// Children are recursively removed
    pub fn remove(&mut self, key: DefaultKey) {
        let children = self.nodes[key].children.clone();
        self.nodes.remove(key);
        for child_key in children {
            self.remove(child_key);
        }
    }

    // ********************************************************************* //
    //                          1ST PASS: FIXED WIDTHS                       //
    // ********************************************************************* //

    /// Recursive fixed width computation pass.
    /// Because this is the first pass, it is responsible for creating nodes
    fn recurse_fixed_width(&mut self, div: &Div) -> DefaultKey {
        let mut child_keys = Vec::with_capacity(div.children.len());
        for child in &div.children {
            let child_key = self.recurse_fixed_width(child);
            child_keys.push(child_key);
        }

        let rect = Rect {
            x: 0,
            y: 0,
            width: match div.width {
                Size::Fixed(w) => w,
                Size::Fit => 0,
            },
            height: 0,
            children: child_keys,
        };

        self.nodes.insert(rect)
    }

    // ********************************************************************* //
    //                          2ND PASS: GROW WIDTHS                        //
    // ********************************************************************* //

    // Recursive grow width computation pass.
    fn recurse_grow_width(&mut self, key: DefaultKey, div: &Div) {}
}
