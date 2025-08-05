//! Layout computation

use crate::{div::Div, layout::Layout, rect::Rect, size::Size};

/// Rect arena for layout computation.
/// We store rects and their children in different attributes
/// to avoid recursive references, and to allow mutably borrowing
/// one while still being able to access the others.
pub struct Arena {
    /// Rect nodes
    pub nodes: Vec<Rect>,
    /// Rect children
    pub children: Vec<Vec<usize>>,
    /// Keep track of free slots in the vector
    pub(crate) free: Vec<usize>,
}

impl Arena {
    /// Create a new empty arena
    pub fn new() -> Self {
        Arena {
            nodes: Vec::new(),
            children: Vec::new(),
            free: Vec::new(),
        }
    }

    /// Compute the layout for a given root node
    pub fn compute(&mut self, root: &Div) -> usize {
        // 1st pass: compute fixed widths (top-down)
        let root_key = self.compute_fixed_width(root);

        // 2nd pass: compute grow widths (bottom-up)
        self.compute_grow_width(root_key, root);

        // 3rd pass: wrap text (todo)

        // 4th pass: compute fixed heights (top-down)
        self.compute_fixed_height(root_key, root);

        // 5th pass: compute grow heights (bottom-up)
        self.compute_grow_height(root_key, root);

        // Return root key
        root_key
    }

    // ********************************************************************* //
    //                          1ST PASS: FIXED WIDTHS                       //
    // ********************************************************************* //

    /// Recursive fixed width computation pass.
    /// Because this is the first pass, it is responsible for creating nodes
    fn compute_fixed_width(&mut self, div: &Div) -> usize {
        // Create rect for the current div
        let key = self.insert(
            Rect {
                x: 0,
                y: 0,
                width: match div.width {
                    Size::Fixed(w) => w,
                    Size::Fit => 0,
                },
                height: 0,
            },
            div.children.len(),
        );

        for child in &div.children {
            let child_key = self.compute_fixed_width(child);
            self.children[key].push(child_key);
        }

        key
    }

    // ********************************************************************* //
    //                          2ND PASS: GROW WIDTHS                        //
    // ********************************************************************* //

    // Recursive grow width computation pass.
    fn compute_grow_width(&mut self, key: usize, div: &Div) {
        recurse_grow_width(key, div, &mut self.nodes, &self.children);
    }

    // ********************************************************************* //
    //                         4TH PASS: FIXED HEIGHTS                       //
    // ********************************************************************* //

    // Recursive fixed height computation pass.
    fn compute_fixed_height(&mut self, key: usize, div: &Div) {
        recurse_fixed_height(key, div, &mut self.nodes, &self.children);
    }

    // ********************************************************************* //
    //                          5TH PASS: GROW WIDTHS                        //
    // ********************************************************************* //

    // Recursive grow height computation pass.
    fn compute_grow_height(&mut self, key: usize, div: &Div) {
        recurse_grow_height(key, div, &mut self.nodes, &self.children);
    }

    // ********************************************************************* //
    //                            SLOTMAP UTILITIES                          //
    // ********************************************************************* //

    /// Insert a new rectangle into the arena and return its key.
    /// An empty children vector is created for it with the given capacity.
    fn insert(&mut self, rect: Rect, capacity: usize) -> usize {
        if let Some(free_key) = self.free.pop() {
            // Reuse a free slot
            self.nodes[free_key] = rect;
            self.children[free_key].clear();
            self.children[free_key].resize(capacity, 0);
            free_key
        } else {
            // Create a new slot
            let key = self.nodes.len();
            self.nodes.push(rect);
            self.children.push(Vec::with_capacity(capacity));
            key
        }
    }

    /// Remove a rectangle from the arena by its key.
    /// Its children are also removed.
    pub fn remove(&mut self, key: usize) {
        recurse_free(key, &mut self.free, &self.children);
    }
}

/// Recursively "free" nodes in the arena.
/// This needs an auxiliary function for fine-grained borrowing.
fn recurse_free(node: usize, free: &mut Vec<usize>, children: &[Vec<usize>]) {
    // Recursively free children nodes
    for &child in &children[node] {
        recurse_free(child, free, children);
    }
    // Add the current node to the free list
    free.push(node);
}

/// Recursively compute grow widths in a bottom-up manner.
/// Is an auxiliary function for fine-grained borrowing.
fn recurse_grow_width(node: usize, div: &Div, nodes: &mut [Rect], children: &[Vec<usize>]) {
    // Recurse children sizes
    for (index, div) in children[node].iter().zip(div.children.iter()) {
        recurse_grow_width(*index, div, nodes, children);
    }

    let children = &children[node];

    // Compute current node size from children if size policy is Fit.
    match div.width {
        Size::Fit => {
            let mut width = 0;
            match div.layout {
                // Vertical layout: width is max of children widths + margins
                Layout::Vertical => {
                    for (index, child) in children.iter().zip(div.children.iter()) {
                        width =
                            width.max(nodes[*index].width + child.margin.left + child.margin.right);
                    }
                }
                // Horizontal layout: width is sum of children widths + margins
                Layout::Horizontal => {
                    for (index, child) in children.iter().zip(div.children.iter()) {
                        width += nodes[*index].width + child.margin.left + child.margin.right;
                    }
                }
            }
            // Add parent padding
            width += div.padding.left + div.padding.right;

            // Assign back to the current node
            nodes[node].width = width;
        }
        // If fixed size, nothing to be done
        _ => {}
    }
}

/// Recursive fixed height computation pass.
/// Is an auxiliary function for fine-grained borrowing.
fn recurse_fixed_height(node: usize, div: &Div, nodes: &mut [Rect], children: &[Vec<usize>]) {
    nodes[node].height = match div.height {
        Size::Fixed(h) => h,
        Size::Fit => 0, // Will be computed later
    };

    // Recurse children sizes
    for (index, child_div) in children[node].iter().zip(div.children.iter()) {
        recurse_fixed_height(*index, child_div, nodes, children);
    }
}

/// Compute the height of the current node based on its children.
/// This is an auxiliary function for fine-grained borrowing.
fn recurse_grow_height(node: usize, div: &Div, nodes: &mut [Rect], children: &[Vec<usize>]) {
    // Recurse children sizes
    for (index, child_div) in children[node].iter().zip(div.children.iter()) {
        recurse_grow_height(*index, child_div, nodes, children);
    }

    let children = &children[node];

    // Compute current node size from children if size policy is Fit.
    match div.height {
        Size::Fit => {
            let mut height = 0;
            match div.layout {
                // Vertical layout: height is sum of children heights + margins
                Layout::Vertical => {
                    for (index, child) in children.iter().zip(div.children.iter()) {
                        height += nodes[*index].height + child.margin.top + child.margin.bottom;
                    }
                }
                // Horizontal layout: height is max of children heights + margins
                Layout::Horizontal => {
                    for (index, child) in children.iter().zip(div.children.iter()) {
                        height = height
                            .max(nodes[*index].height + child.margin.top + child.margin.bottom);
                    }
                }
            }
            // Add parent padding
            height += div.padding.top + div.padding.bottom;

            // Assign back to the current node
            nodes[node].height = height;
        }
        // If fixed size, nothing to be done
        _ => {}
    }
}
