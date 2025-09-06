//! DOM structure to manage UI elements

use rice_layout::{Layout, Rect, compute_layout};

use crate::{Style, StyleRules, mouse::recurse_mouse};

/// Main arena DOM
pub struct DOM {
    /// Layout rules
    pub layouts: Vec<Layout>,
    /// Computed sizes & positions
    pub rects: Vec<Rect>,
    /// Children indices for each node
    pub children: Vec<Vec<usize>>,
    /// Computed styles from style rules (ready to be written to a buffer)
    pub styles: Vec<Style>,
    /// Style rules for each node
    pub styles_rules: Vec<StyleRules>,

    /// Hover state
    pub hovered: Option<usize>,

    /// Dirty nodes that must be redrawn with their children
    pub dirty: Vec<usize>,
    /// Nodes that need to be redrawn, in ascending z-index (computed from dirty)
    pub redraw: Vec<usize>,
}

impl DOM {
    /// Create a new empty arena
    pub fn new() -> Self {
        Self {
            layouts: Vec::new(),
            rects: Vec::new(),
            children: Vec::new(),
            styles: Vec::new(),
            styles_rules: Vec::new(),

            hovered: None,
            dirty: Vec::new(),
            redraw: Vec::new(),
        }
    }

    /// Recompute the entire layout starting from the given root node
    pub fn compute_layout(&mut self, root: usize) {
        compute_layout(root, &self.layouts, &mut self.rects, &self.children);
    }

    /// Insert a root node into the arena
    pub fn insert(&mut self, layout: Layout, style_rules: StyleRules) -> usize {
        let mut style = Style::default();
        style_rules.apply_default(&mut style);
        self.layouts.push(layout);
        self.rects.push(Rect::default());
        self.children.push(vec![]);
        self.styles_rules.push(style_rules);
        self.styles.push(style);
        self.layouts.len() - 1
    }

    /// Insert a child node into the arena
    pub fn insert_child(&mut self, layout: Layout, style: StyleRules, parent: usize) -> usize {
        let index = self.insert(layout, style);
        self.children[parent].push(index);
        index
    }

    /// Handle mouse position (hover state, dirty nodes, etc)
    pub fn handle_mouse(&mut self, mouse: [i32; 2]) {
        // 1. Get the index of the newly hovered node
        let index = recurse_mouse(0, &self.rects, &self.children, &mouse);

        if index == self.hovered {
            return;
        }

        // 2. If different, handle drawing the new and redrawing the old
        if let Some(old) = self.hovered
            && !self.styles_rules[old].hovered.is_empty()
        {
            // TODO: completely reverse hover rules
            self.styles_rules[old].apply_default(&mut self.styles[old]);
            self.dirty.push(old);
        }

        if let Some(new) = index
            && !self.styles_rules[new].hovered.is_empty()
        {
            self.styles_rules[new].apply_hovered(&mut self.styles[new]);
            self.dirty.push(new);
        }
        self.hovered = index;
    }

    /// Compute the indices of the rectangles that need to be redrawn, in ascending z-index
    /// TODO: replace naive recursion to avoid duplicates
    pub fn compute_redraw(&mut self) -> &[usize] {
        self.redraw.clear();

        for &index in &self.dirty {
            recurse_children(index, &self.children, &mut self.redraw);
        }

        &self.redraw
    }
}

/// Recursively add children indices of the given index to the output vector
fn recurse_children(index: usize, children: &[Vec<usize>], out: &mut Vec<usize>) {
    out.push(index);
    for &child in &children[index] {
        recurse_children(child, children, out);
    }
}
