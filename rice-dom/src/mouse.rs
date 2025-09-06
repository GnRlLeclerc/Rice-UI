//! Mouse handling

use rice_layout::Rect;

/// Recursively find the childmost rectangle containing the given point.
/// The root rectangle (screen) is guaranteed to contain the point.
pub fn recurse_mouse(
    index: usize,
    rects: &[Rect],
    children: &[Vec<usize>],
    mouse: &[i32; 2],
) -> Option<usize> {
    // 1. Check if the point is inside this rect
    let rect = &rects[index];
    if mouse[0] < rect.position[0]
        || rect.size[0] + rect.position[0] < mouse[0]
        || mouse[1] < rect.position[1]
        || rect.size[1] + rect.position[1] < mouse[1]
    {
        // if not, stop
        return None;
    }

    // 2. Recurse through children
    for idx in &children[index] {
        if let Some(idx) = recurse_mouse(*idx, rects, children, mouse) {
            return Some(idx);
        }
    }

    Some(index)
}
