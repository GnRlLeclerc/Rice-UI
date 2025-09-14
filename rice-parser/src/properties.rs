//! Parse property nodes

use core::str;

use rice_dom::{StyleProp, StyleSheet, StyleValue};
use rice_layout::Layout;
use tree_sitter::Node;

use crate::values::{parse_color, parse_size};

/// Parse a property node and update the given layout and stylesheet accordingly.
pub fn parse_property(
    node: Node,
    content: &[u8],
    layout: &mut Layout,
    stylesheet: &mut StyleSheet,
) {
    // Populate propname & value
    let mut propname = "";
    let mut value = None;

    for child in node.named_children(&mut node.walk()) {
        match child.kind() {
            "propname" => {
                propname = str::from_utf8(&content[child.byte_range()]).unwrap();
            }
            "value" => {
                value = Some(child.named_child(0).expect("Expected value child node"));
            }
            _ => unreachable!("Unexpected node kind for property: {}", child.kind()),
        }
    }

    // Match possibilities
    match propname.as_ref() {
        "width" => {
            layout.size[0] = parse_size(value.expect("Expected value node"), content);
        }
        "height" => {
            layout.size[1] = parse_size(value.expect("Expected value node"), content);
        }
        "bg_color" => {
            stylesheet.default.insert(
                StyleProp::BackgroundColor,
                StyleValue::Color(parse_color(value.expect("Expected value node"), content)),
            );
        }
        _ => {
            panic!("Unknown property name: {}", propname);
        }
    }
}
