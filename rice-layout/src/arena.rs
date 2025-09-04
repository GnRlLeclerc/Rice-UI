//! Layout engine using arena for storage

use crate::{Direction, Layout, Rect};
use crate::{Size, utils::*};

pub struct Arena {
    /// Layout rules
    pub layouts: Vec<Layout>,
    /// Computed sizes & positions
    pub rects: Vec<Rect>,
    /// Children indices for each node
    pub children: Vec<Vec<usize>>,
    /// Reusable indices
    free: Vec<usize>,
}

impl Arena {
    /// Create a new empty arena
    pub fn new() -> Self {
        Self {
            layouts: Vec::new(),
            rects: Vec::new(),
            children: Vec::new(),
            free: Vec::new(),
        }
    }

    /// Recompute the entire layout starting from the given root node
    pub fn compute_layout(&mut self, root: usize) {
        // 1st pass: compute fixed widths (top-down)
        recurse_fixed(root, &self.layouts, &mut self.rects, &self.children, 0);
        // 2nd pass: compute expand widths (bottom-up)
        recurse_grow_width(root, &self.layouts, &mut self.rects, &self.children);
        // 3rd pass: compute text height after wrapping (todo)
        // 4th pass: compute fixed heights (top-down)
        recurse_fixed(root, &self.layouts, &mut self.rects, &self.children, 1);
        // 5th pass: compute expand heights (bottom-up)
        recurse_grow_height(root, &self.layouts, &mut self.rects, &self.children);
        // 6th pass: compute positions (top-down)
        recurse_positions(root, &self.layouts, &mut self.rects, &self.children);
    }

    /// Insert a root node into the arena
    pub fn insert(&mut self, layout: Layout) -> usize {
        let rect = Rect::default();

        if let Some(index) = self.free.pop() {
            self.layouts[index] = layout;
            self.rects[index] = rect;
            self.children[index].clear();
            index
        } else {
            self.layouts.push(layout);
            self.rects.push(rect);
            self.children.push(vec![]);
            self.layouts.len() - 1
        }
    }

    /// Insert a child node into the arena
    pub fn insert_child(&mut self, layout: Layout, parent: usize) -> usize {
        let index = self.insert(layout);
        self.children[parent].push(index);
        index
    }

    /// Remove a node and all its children from the arena
    pub fn remove(&mut self, index: usize) {
        recurse_remove(index, &mut self.free, &self.children);
    }
}

/// Compute fixed & percent dimensions in a top-down recursive way
fn recurse_fixed(
    node: usize,
    layouts: &[Layout],
    rects: &mut [Rect],
    children: &[Vec<usize>],
    dim: usize,
) {
    // 1. Compute fixed dimensions for current node + dimensions widths for children
    fixed(node, rects, layouts, children, dim);

    // 2. Recurse children
    for &idx in &children[node] {
        recurse_fixed(idx, layouts, rects, children, dim);
    }
}

/// Compute grow widths in a bottom-up recursive way
fn recurse_grow_width(
    node: usize,
    layouts: &[Layout],
    rects: &mut [Rect],
    children: &[Vec<usize>],
) {
    // 1. Recurse children
    for &idx in &children[node] {
        recurse_grow_width(idx, layouts, rects, children);
    }

    // 2. If this node is "fit", compute its size based on children + direction
    if let Size::Fit = layouts[node].size[0] {
        match layouts[node].direction {
            Direction::Horizontal(_) => fit_along(node, rects, layouts, children, 0),
            Direction::Vertical(_) => fit_across(node, rects, layouts, children, 0),
        }
    }

    // 3. Expand all children of this node that are expandable
    match layouts[node].direction {
        Direction::Horizontal(_) => expand_along(node, rects, layouts, children, 0),
        Direction::Vertical(_) => expand_across(node, rects, layouts, children, 0),
    }
}

/// Compute grow heights in a bottom-up recursive way
fn recurse_grow_height(
    node: usize,
    layouts: &[Layout],
    rects: &mut [Rect],
    children: &[Vec<usize>],
) {
    // 1. Recurse children
    for &idx in &children[node] {
        recurse_grow_height(idx, layouts, rects, children);
    }

    // 2. If this node is "fit", compute its size based on children + direction
    if let Size::Fit = layouts[node].size[1] {
        match layouts[node].direction {
            Direction::Horizontal(_) => fit_across(node, rects, layouts, children, 1),
            Direction::Vertical(_) => fit_along(node, rects, layouts, children, 1),
        }
    }

    // 3. Expand all children of this node that are expandable
    match layouts[node].direction {
        Direction::Horizontal(_) => expand_across(node, rects, layouts, children, 1),
        Direction::Vertical(_) => expand_along(node, rects, layouts, children, 1),
    }
}

fn recurse_positions(node: usize, layouts: &[Layout], rects: &mut [Rect], children: &[Vec<usize>]) {
    // 1. Compute positions of childre
    positions(node, rects, layouts, children);

    // 2. Recurse children
    for &idx in &children[node] {
        recurse_positions(idx, layouts, rects, children);
    }
}

fn recurse_remove(node: usize, free: &mut Vec<usize>, children: &[Vec<usize>]) {
    free.push(node);

    for &child in &children[node] {
        recurse_remove(child, free, children);
    }
}
