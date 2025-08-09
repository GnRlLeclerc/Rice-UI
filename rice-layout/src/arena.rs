//! Layout computation

use crate::{Gap, div::Div, layout::Layout, rect::Rect, size::Size};

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
        // Initialize the arena with the root div
        let root_key = self.initialize(root);

        // 1st pass: compute fixed widths (top-down)
        recurse_fixed_width(root_key, root, &mut self.nodes, &self.children);

        // 2nd pass: compute grow widths (bottom-up)
        recurse_grow_width(root_key, root, &mut self.nodes, &self.children);

        // 3rd pass: wrap text (todo)

        // 4th pass: compute fixed heights (top-down)
        recurse_fixed_height(root_key, root, &mut self.nodes, &self.children);

        // 5th pass: compute grow heights (bottom-up)
        recurse_grow_height(root_key, root, &mut self.nodes, &self.children);

        // 6th pass: compute positions (top-down)
        recurse_positions(root_key, root, &mut self.nodes, &self.children);

        // Return root key
        root_key
    }

    /// Initialize the arena to create corresponding rects for the given root div
    fn initialize(&mut self, div: &Div) -> usize {
        // Create a new rectangle for the root div
        let key = self.insert(Rect::default(), div.children.len());

        // Initialize children recursively
        for child in &div.children {
            let child_key = self.initialize(child);
            self.children[key].push(child_key);
        }

        key
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

// ************************************************************************* //
//                            1ST PASS: FIXED WIDTHS                         //
// ************************************************************************* //

/// Recursively compute fixed widths in a top-down manner.
/// Is an auxiliary function for fine-grained borrowing.
fn recurse_fixed_width(node: usize, div: &Div, nodes: &mut [Rect], children: &[Vec<usize>]) {
    // Set the width of the current node if it is fixed
    nodes[node].width = match div.width {
        Size::Fixed(w) => w,
        Size::Fit | Size::Expand(_) => 0, // Will be computed later
    };

    // Recurse children sizes
    for (index, child_div) in children[node].iter().zip(div.children.iter()) {
        recurse_fixed_width(*index, child_div, nodes, children);
    }
}

// ************************************************************************* //
//                            2ND PASS: GROW WIDTHS                          //
// ************************************************************************* //

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
                Layout::Vertical(_) => {
                    for (index, child) in children.iter().zip(div.children.iter()) {
                        width =
                            width.max(nodes[*index].width + child.margin.left + child.margin.right);
                    }
                }
                // Horizontal layout: width is sum of children widths + margins
                Layout::Horizontal(_) => {
                    for (index, child) in children.iter().zip(div.children.iter()) {
                        width += nodes[*index].width + child.margin.left + child.margin.right
                    }
                }
            }
            // Add parent padding
            width += div.padding.left + div.padding.right;

            // Add gap to width if children are disposed horizontally
            if let Gap::Fixed(gap) = div.gap
                && let Layout::Horizontal(_) = div.layout
            {
                width += gap * (children.len().saturating_sub(1)) as i32;
            }

            // Assign back to the current node
            nodes[node].width = div.clip_width(width);
        }
        // If fixed size, nothing to be done
        _ => {}
    }

    if children.is_empty() {
        return;
    }

    // Compute leftover space
    let mut available = nodes[node].width - div.padding.left - div.padding.right;

    match div.layout {
        // Vertical layout: expand all children to fit all available width
        Layout::Vertical(_) => {
            children
                .iter()
                .zip(div.children.iter())
                .for_each(|(&index, child)| {
                    if let Size::Expand(_) = child.width {
                        nodes[index].width =
                            child.clip_width(available - child.margin.left - child.margin.right);
                    }
                });
        }
        // Horizontal layout: divide all available width among children
        Layout::Horizontal(_) => {
            children
                .iter()
                .zip(div.children.iter())
                .for_each(|(&index, child)| {
                    available -= nodes[index].width + child.margin.left + child.margin.right;
                });
            if let Gap::Fixed(gap) = div.gap {
                available -= gap * (children.len().saturating_sub(1)) as i32;
            }

            let mut expandables: Vec<(usize, &Div, f32)> = children
                .iter()
                .zip(div.children.iter())
                .filter_map(|(index, div)| match div.width {
                    Size::Expand(fr) => {
                        // Reset width to 0 (previously set to min width for fit computation)
                        nodes[*index].width = 0;
                        Some((*index, div, fr))
                    }
                    _ => None,
                })
                .collect();

            while available != 0 && !expandables.is_empty() {
                // 1. Compute total fr of all expandables
                let total_fr: f32 = expandables.iter().map(|(_, _, fr)| *fr).sum();
                if total_fr == 0.0 {
                    break;
                }

                // 2. Distribute remaining space to all expandables
                for (index, _, fr) in &expandables {
                    nodes[*index].width += (available as f32 * fr / total_fr).round() as i32;
                }
                available = 0;

                // 3. Clip widths and put the extra space back to available
                let mut i = 0;
                while i < expandables.len() {
                    let (index, div, _) = &expandables[i];
                    let width = div.clip_width(nodes[*index].width);
                    let delta = nodes[*index].width - width;
                    if delta != 0 {
                        // Width was clipped, remove this element from the list
                        available += delta;
                        nodes[*index].width = width;
                        expandables.swap_remove(i);
                    } else {
                        i += 1;
                    }
                }
            }
        }
    }
}

// ************************************************************************* //
//                           4TH PASS: FIXED HEIGHTS                         //
// ************************************************************************* //

/// Recursive fixed height computation pass.
/// Is an auxiliary function for fine-grained borrowing.
fn recurse_fixed_height(node: usize, div: &Div, nodes: &mut [Rect], children: &[Vec<usize>]) {
    nodes[node].height = match div.height {
        Size::Fixed(h) => h,
        Size::Fit | Size::Expand(_) => 0, // Will be computed later
    };

    // Recurse children sizes
    for (index, child_div) in children[node].iter().zip(div.children.iter()) {
        recurse_fixed_height(*index, child_div, nodes, children);
    }
}

// ************************************************************************* //
//                            5TH PASS: GROW WIDTHS                          //
// ************************************************************************* //

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
                Layout::Vertical(_) => {
                    for (index, child) in children.iter().zip(div.children.iter()) {
                        height += nodes[*index].height + child.margin.top + child.margin.bottom
                    }
                }
                // Horizontal layout: height is max of children heights + margins
                Layout::Horizontal(_) => {
                    for (index, child) in children.iter().zip(div.children.iter()) {
                        height = height
                            .max(nodes[*index].height + child.margin.top + child.margin.bottom);
                    }
                }
            }
            // Add parent padding
            height += div.padding.top + div.padding.bottom;

            // Add gap to height if children are disposed vertically
            if let Gap::Fixed(gap) = div.gap
                && let Layout::Vertical(_) = div.layout
            {
                height += gap * (children.len().saturating_sub(1)) as i32;
            }

            // Assign back to the current node
            nodes[node].height = div.clip_height(height);
        }
        // If fixed size, nothing to be done
        _ => {}
    }

    if children.is_empty() {
        return;
    }

    // Compute leftover space
    let mut available = nodes[node].height - div.padding.top - div.padding.bottom;

    match div.layout {
        // Vertical layout: divide all available width among children
        Layout::Vertical(_) => {
            children
                .iter()
                .zip(div.children.iter())
                .for_each(|(&index, child)| {
                    available -= nodes[index].height + child.margin.top + child.margin.bottom;
                });
            if let Gap::Fixed(gap) = div.gap {
                available -= gap * (children.len().saturating_sub(1)) as i32;
            }

            let mut expandables: Vec<(usize, &Div, f32)> = children
                .iter()
                .zip(div.children.iter())
                .filter_map(|(index, div)| match div.height {
                    Size::Expand(fr) => {
                        // Reset height to 0 (previously set to min height for fit computation)
                        nodes[*index].height = 0;
                        Some((*index, div, fr))
                    }
                    _ => None,
                })
                .collect();

            while available != 0 && !expandables.is_empty() {
                // 1. Compute total fr of all expandables
                let total_fr: f32 = expandables.iter().map(|(_, _, fr)| *fr).sum();
                if total_fr == 0.0 {
                    break;
                }

                // 2. Distribute remaining space to all expandables
                for (index, _, fr) in &expandables {
                    nodes[*index].height += (available as f32 * fr / total_fr).round() as i32;
                }
                available = 0;

                // 3. Clip heights and put the extra space back to available
                let mut i = 0;
                while i < expandables.len() {
                    let (index, div, _) = &expandables[i];
                    let height = div.clip_height(nodes[*index].height);
                    let delta = nodes[*index].height - height;
                    if delta != 0 {
                        // Height was clipped, remove this element from the list
                        available += delta;
                        nodes[*index].height = height;
                        expandables.swap_remove(i);
                    } else {
                        i += 1;
                    }
                }
            }
        }
        // Horizontal layout: expand all children to fit all available height
        Layout::Horizontal(_) => {
            children
                .iter()
                .zip(div.children.iter())
                .for_each(|(&index, child)| {
                    if let Size::Expand(_) = child.height {
                        nodes[index].height =
                            child.clip_height(available - child.margin.top - child.margin.bottom);
                    }
                });
        }
    }
}

// ************************************************************************* //
//                              6TH PASS: POSITIONS                          //
// ************************************************************************* //

/// Compute the position of the children of the current node.
/// This is an auxiliary function for fine-grained borrowing.
fn recurse_positions(node: usize, div: &Div, nodes: &mut [Rect], children: &[Vec<usize>]) {
    let parent_x = &nodes[node].x + div.padding.left;
    let parent_y = &nodes[node].y + div.padding.top;

    let remaining = div.layout.remaining_space(node, div, nodes, children);
    let gap = match div.gap {
        Gap::Fixed(g) => g,
        Gap::Auto => remaining / div.children.len() as i32,
    };

    match div.layout {
        // Vertical layout: accumulate heights and restart widths from parent
        Layout::Vertical(align) => {
            let mut y = parent_y;
            for (&index, child) in children[node].iter().zip(div.children.iter()) {
                // Set child position
                y += child.margin.top;
                nodes[index].x = align.position_x(&nodes[node], &nodes[index], div, child);
                nodes[index].y = y;

                // Recurse child
                recurse_positions(index, child, nodes, children);

                // Update remaining position
                y += nodes[index].height + child.margin.bottom + gap;
            }
        }
        // Horizontal layout: accumulate widths and restart height from parent
        Layout::Horizontal(align) => {
            let mut x = parent_x;
            for (&index, child) in children[node].iter().zip(div.children.iter()) {
                // Set child position
                x += child.margin.left;
                nodes[index].x = x;
                nodes[index].y = align.position_y(&nodes[node], &nodes[index], div, child);

                // Recurse child
                recurse_positions(index, child, nodes, children);

                // Update remaining position
                x += nodes[index].width + child.margin.right + gap;
            }
        }
    }
}
