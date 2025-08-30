//! Computation utilities

use crate::{Align, Direction, Gap, Layout, Rect, Size};

/// Compute fixed & percent sizes for a component along a given dimension
pub fn fixed(
    node: usize,
    rects: &mut [Rect],      // All rects
    layouts: &[Layout],      // All layouts
    children: &[Vec<usize>], // All children
    dim: usize,
) {
    // 1. Set the current element's fixed size
    if let Size::Fixed(size) = layouts[node].size[dim] {
        rects[node].size[dim] = layouts[node].clip_size(dim, size);
    }

    // 2. Compute percent sizes for children
    for &idx in &children[node] {
        if let Size::Percent(fr) = layouts[idx].size[dim] {
            let space = rects[node].size[dim]
                - layouts[node].padding.start[dim]
                - layouts[node].padding.end[dim];

            rects[idx].size[dim] = layouts[idx].clip_size(dim, (space as f32 * fr) as i32);
        }
    }
}

/// Compute a fit size for a component along a given dimension
pub fn fit_along(
    node: usize,
    rects: &mut [Rect],      // All rects
    layouts: &[Layout],      // All layouts
    children: &[Vec<usize>], // All children
    dim: usize,
) {
    // Start with self padding
    let mut total = layouts[node].padding.start[dim] + layouts[node].padding.end[dim];

    for idx in &children[node] {
        let layout = &layouts[*idx];
        let rect = &rects[*idx];
        // Add margin + size for each child
        total += layout.margin.start[dim] + rect.size[dim] + layout.margin.end[dim];
    }

    // Add gaps if needed
    if let Gap::Fixed(size) = layouts[node].gap {
        total += children[node].len().saturating_sub(1) as i32 * size;
    }

    // Set the size, clipped to min/max
    rects[node].size[dim] = layouts[node].clip_size(dim, total);
}

/// Compute a fit size for a component along a given dimension
pub fn fit_across(
    node: usize,
    rects: &mut [Rect],      // All rects
    layouts: &[Layout],      // All layouts
    children: &[Vec<usize>], // All children
    dim: usize,
) {
    let total = layouts[node].padding.start[dim] + layouts[node].padding.end[dim];

    // Compute the max size of children along this dimension
    let max_child_size = children[node]
        .iter()
        .map(|&idx| {
            layouts[idx].margin.start[dim] + rects[idx].size[dim] + layouts[idx].margin.end[dim]
        })
        .max()
        .unwrap_or(0);

    // Set the size, clipped to min/max
    rects[node].size[dim] = layouts[node].clip_size(dim, total + max_child_size);
}

/// Compute the expand sizes for a component's children along a given dimension
///
/// Computes the remaining size after all fixed / percent / fit components have been substracted,
/// and shares it among the expandable components iteratively, to respect min/max constraints.
pub fn expand_along(
    node: usize,
    rects: &mut [Rect],      // All rects
    layouts: &[Layout],      // All layouts
    children: &[Vec<usize>], // All children
    dim: usize,
) {
    let mut total_fr = 0.0;
    let mut indexes = Vec::new(); // Track indexes of expandable components
    let mut occupied = 0;

    // 1. Compute total shared fraction & expandable component indexes
    for &idx in children[node].iter() {
        let layout = &layouts[idx];

        // Add margins
        occupied += layout.margin.start[dim] + layout.margin.end[dim];

        // Track expendable, or add occupied size
        match layout.size[dim] {
            Size::Expand(fraction) => {
                total_fr += fraction;
                indexes.push(idx);
            }
            _ => occupied += rects[idx].size[dim],
        }
    }

    let gap = match layouts[node].gap {
        Gap::Fixed(size) => size * children[node].len().saturating_sub(1) as i32,
        Gap::Auto => 0,
    };

    let mut remaining = rects[node].size[dim]
        - layouts[node].padding.start[dim]
        - layouts[node].padding.end[dim]
        - gap
        - occupied;

    while !indexes.is_empty() && remaining != 0 {
        // 1. Attribute remaining size depending on fractions
        for i in 0..indexes.len() {
            let idx = indexes[i];
            let fr = match layouts[idx].size[dim] {
                Size::Expand(fr) => fr,
                _ => unreachable!("all elements in expand_along indexes should be expandable"),
            };

            rects[idx].size[dim] += (remaining as f32 * fr / total_fr).round() as i32
        }

        remaining = 0;

        // 2. Clip to min/max layouts, and remove component that reached their limits
        let mut i = 0;
        while i < indexes.len() {
            let idx = indexes[i];
            let size = rects[idx].size[dim];
            let clipped = layouts[idx].clip_size(dim, size);
            if size != clipped {
                rects[idx].size[dim] = clipped;
                remaining += size - clipped;
                indexes.swap_remove(i);
                total_fr -= match layouts[idx].size[dim] {
                    Size::Expand(fr) => fr,
                    _ => unreachable!("all elements in expand_along indexes should be expandable"),
                };
            } else {
                i += 1;
            }
        }
    }
}

/// Compute the expand sizes for a component's children across a given dimension
///
/// Expands all expandable components to fit the space within the parent component
pub fn expand_across(
    node: usize,
    rects: &mut [Rect],      // All rects
    layouts: &[Layout],      // All layouts
    children: &[Vec<usize>], // All children
    dim: usize,
) {
    let space =
        rects[node].size[dim] - layouts[node].padding.start[dim] - layouts[node].padding.end[dim];

    for &idx in &children[node] {
        if let Size::Expand(_) = layouts[idx].size[dim] {
            rects[idx].size[dim] = layouts[idx].clip_size(
                dim,
                space - layouts[idx].margin.start[dim] - layouts[idx].margin.end[dim],
            );
        }
    }
}

/// Compute the positions of a component's children in a top-down way,
/// for all directions at once
pub fn positions(
    node: usize,
    rects: &mut [Rect],      // All rects
    layouts: &[Layout],      // All layouts
    children: &[Vec<usize>], // All children
) {
    // Compute dimension indexes
    let [along, across] = match layouts[node].direction {
        Direction::Horizontal(_) => [0_usize, 1_usize],
        Direction::Vertical(_) => [1_usize, 0_usize],
    };
    let align = layouts[node].direction.align();

    // Compute gap between children along the layout direction
    let gap = gap_along(layouts[node].gap, node, rects, layouts, children, along);

    // Starting position along the layout direction
    let mut pos_along = rects[node].position[along] + layouts[node].padding.start[along];

    for &idx in &children[node] {
        // Position across the direction (independent from other children)
        rects[idx].position[across] = position_across(
            align,
            &layouts[node],
            &rects[node],
            &layouts[idx],
            &rects[idx],
            across,
        );

        // Position along the direction (accumulate child after child)
        pos_along += layouts[idx].margin.start[along];
        rects[idx].position[along] = pos_along;
        pos_along += rects[idx].size[along] + layouts[idx].margin.end[along] + gap;
    }
}

/// Returns the position of a child element across the layout direction of its parent
/// given the alignment layout
pub fn position_across(
    align: Align,
    parent_layout: &Layout,
    parent_rect: &Rect,
    child_layout: &Layout,
    child_rect: &Rect,
    dim: usize,
) -> i32 {
    match align {
        // Stick to start: only parent padding & child margin
        Align::Start => {
            parent_rect.position[dim]
                + parent_layout.padding.start[dim]
                + child_layout.margin.start[dim]
        }
        // Center: center child according to its size + size within parent padding
        Align::Center => {
            parent_rect.position[dim]
                + parent_layout.padding.start[dim]
                + child_layout.margin.start[dim]
                + (parent_rect.size[dim]
                    - parent_layout.padding.start[dim]
                    - parent_layout.padding.end[dim]
                    - child_rect.size[dim]
                    - child_layout.margin.start[dim]
                    - child_layout.margin.end[dim])
                    / 2
        }
        // Stick to end: only parent padding & child margin
        Align::End => {
            parent_rect.position[dim] + parent_rect.size[dim]
                - parent_layout.padding.end[dim]
                - child_layout.margin.end[dim]
                - child_rect.size[dim]
        }
    }
}

/// Returns the gap value between children in a layout, along the given dimension
fn gap_along(
    gap: Gap,
    node: usize,
    rects: &[Rect],          // All rects
    layouts: &[Layout],      // All layouts
    children: &[Vec<usize>], // All children
    dim: usize,
) -> i32 {
    match gap {
        Gap::Fixed(size) => size,
        Gap::Auto => {
            let mut remaining = rects[node].size[dim]
                - layouts[node].padding.start[dim]
                - layouts[node].padding.end[dim];

            // Remove children sizes with their margins
            for idx in &children[node] {
                let layout = &layouts[*idx];
                let rect = &rects[*idx];
                remaining -= layout.margin.start[dim] + rect.size[dim] + layout.margin.end[dim];
            }

            remaining / children[node].len().saturating_sub(1) as i32
        }
    }
}
