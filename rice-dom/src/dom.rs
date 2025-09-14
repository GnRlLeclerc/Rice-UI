//! DOM structure to manage UI elements

use rice_layout::{Layout, Rect, compute_layout};

use crate::{ComputedStyle, StyleSheet, mouse::recurse_mouse};

/// Main arena DOM
#[derive(Debug)]
pub struct DOM {
    /// Root node index
    pub root: usize,

    /// Layout rules
    pub layouts: Vec<Layout>,
    /// Computed sizes & positions
    pub rects: Vec<Rect>,
    /// Children indices for each node
    pub children: Vec<Vec<usize>>,
    /// Computed styles from style rules (ready to be written to a buffer)
    pub styles: Vec<ComputedStyle>,
    /// Style rules for each node
    pub stylesheets: Vec<StyleSheet>,

    /// Mouse position
    pub mouse: [i32; 2],
    /// Hover state
    pub hovered: Option<usize>,
    /// Clicked state
    pub clicked: Option<usize>,

    /// Dirty nodes that must be redrawn with their children
    pub dirty: Vec<usize>,
    /// Nodes that need to be redrawn, in ascending z-index (computed from dirty)
    pub redraw: Vec<usize>,
}

impl DOM {
    /// Create a new empty arena
    pub fn new() -> Self {
        Self {
            root: 0,

            layouts: Vec::new(),
            rects: Vec::new(),
            children: Vec::new(),
            styles: Vec::new(),
            stylesheets: Vec::new(),

            mouse: [-1, -1],
            hovered: None,
            clicked: None,
            dirty: Vec::new(),
            redraw: Vec::new(),
        }
    }

    /// Recompute the entire layout starting from the given root node
    pub fn compute_layout(&mut self, root: usize) {
        compute_layout(root, &self.layouts, &mut self.rects, &self.children);
    }

    /// Insert a root node into the arena
    pub fn insert(&mut self, layout: Layout, stylesheet: StyleSheet) -> usize {
        let mut style = ComputedStyle::default();
        stylesheet.apply_default(&mut style);
        self.layouts.push(layout);
        self.rects.push(Rect::default());
        self.children.push(vec![]);
        self.stylesheets.push(stylesheet);
        self.styles.push(style);
        self.layouts.len() - 1
    }

    /// Insert a node with children into the arena
    pub fn insert_with_children(
        &mut self,
        layout: Layout,
        style: StyleSheet,
        children: Vec<usize>,
    ) -> usize {
        let index = self.insert(layout, style);
        self.children[index] = children;
        index
    }

    /// Insert a child node into the arena
    pub fn insert_child(&mut self, layout: Layout, style: StyleSheet, parent: usize) -> usize {
        let index = self.insert(layout, style);
        self.children[parent].push(index);
        index
    }

    /// Handle mouse position movement
    pub fn handle_mouse_moved(&mut self, mouse: [i32; 2]) {
        self.mouse = mouse;

        // 1. If something is clicked, ignore hovers
        if self.clicked.is_some() {
            return;
        }

        // 2. Else, get the index of the newly hovered node
        let index = recurse_mouse(0, &self.rects, &self.children, &mouse);

        if index == self.hovered {
            return;
        }

        // 3. If different, handle drawing the new and redrawing the old
        if let Some(old) = self.hovered
            && !self.stylesheets[old].hovered.is_empty()
        {
            self.stylesheets[old].reset_hover(&mut self.styles[old]);
            self.dirty.push(old);
        }

        if let Some(new) = index
            && !self.stylesheets[new].hovered.is_empty()
        {
            self.stylesheets[new].apply_hovered(&mut self.styles[new]);
            self.dirty.push(new);
        }
        self.hovered = index;
    }

    /// Handle mouse clicks
    pub fn handle_mouse_clicked(&mut self, clicked: bool) {
        // Ignore if the state didn't change
        if (self.clicked.is_some() && clicked) || (self.clicked.is_none() && !clicked) {
            return;
        }

        // Get index of currently hovered position
        let index = recurse_mouse(0, &self.rects, &self.children, &self.mouse);

        // If clicked, reset hover state, then apply clicked state
        if clicked {
            // Reset hover state
            if let Some(hovered) = self.hovered {
                self.stylesheets[hovered].reset_hover(&mut self.styles[hovered]);
                self.dirty.push(hovered);
                self.hovered = None;
            }

            self.clicked = index;

            // Apply clicked state
            if let Some(index) = index {
                self.stylesheets[index].apply_clicked(&mut self.styles[index]);
                self.dirty.push(index);
            }
        } else {
            // Reset clicked state
            if let Some(clicked) = self.clicked {
                self.stylesheets[clicked].reset_clicked(&mut self.styles[clicked]);
                self.dirty.push(clicked);
            }

            self.clicked = None;

            // Re-apply hover state if applicable
            if let Some(index) = index {
                self.stylesheets[index].apply_hovered(&mut self.styles[index]);
                self.dirty.push(index);
                self.hovered = Some(index);
            }
        }
    }

    /// Reset clicked and hover states
    pub fn reset_mouse(&mut self) {
        if let Some(clicked) = self.clicked {
            self.stylesheets[clicked].reset_clicked(&mut self.styles[clicked]);
            self.dirty.push(clicked);
            self.clicked = None;
        }
        if let Some(hovered) = self.hovered {
            self.stylesheets[hovered].reset_hover(&mut self.styles[hovered]);
            self.dirty.push(hovered);
            self.hovered = None;
        }
        self.mouse = [-1, -1];
    }

    /// Compute the indices of the rectangles that need to be redrawn, in ascending z-index
    ///
    /// Done by travesring the DOM tree, and adding recursively all indices of subtrees of dirty
    /// nodes. This ensures that parents are drawn before children, and that there are no
    /// duplicates.
    pub fn compute_redraw(&mut self) -> &[usize] {
        self.redraw.clear();

        // Recurse from the root node
        recurse_explore_children(self.root, &self.children, &self.dirty, &mut self.redraw);

        &self.redraw
    }
}

/// Recursively add children indices of the given index to the output vector
fn recurse_add_children(index: usize, children: &[Vec<usize>], out: &mut Vec<usize>) {
    out.push(index);
    for &child in &children[index] {
        recurse_add_children(child, children, out);
    }
}

/// Recursively explore children until a dirty node is found.
/// Dirty subtrees are then explored using `recurse_add_children`.
fn recurse_explore_children(
    index: usize,
    children: &[Vec<usize>],
    dirty: &[usize],
    out: &mut Vec<usize>,
) {
    if dirty.contains(&index) {
        recurse_add_children(index, children, out);
        return;
    }

    for &child in &children[index] {
        recurse_explore_children(child, children, dirty, out);
    }
}
