//! Parse value nodes

use core::str;

use rice_dom::Color;
use rice_layout::Size;
use tree_sitter::Node;

// TODO: propagate error instead of panicking

/// Parse a layout size value from a tree-sitter node.
pub fn parse_size(node: Node, content: &[u8]) -> Size {
    let text = str::from_utf8(&content[node.byte_range()]).unwrap();

    match node.kind() {
        "pixels" => Size::Fixed(text[..text.len() - 2].parse().unwrap()),
        "fraction" => Size::Expand(text[..text.len() - 2].parse().unwrap()),
        "percentage" => Size::Percent(text[..text.len() - 1].parse().unwrap()),
        "identifier" => match text {
            "fit" => Size::Fit,
            _ => panic!("Unknown size identifier: {}", text),
        },
        _ => {
            panic!("Unexpected size node kind: {}", node.kind());
        }
    }
}

/// Parse a color value from a tree-sitter node.
pub fn parse_color(node: Node, content: &[u8]) -> Color {
    match node.kind() {
        "hex_color" => {
            let text = str::from_utf8(&content[node.byte_range()]).unwrap();
            Color::from_hex(text)
        }
        _ => {
            panic!("Unexpected color node kind: {}", node.kind());
        }
    }
}
