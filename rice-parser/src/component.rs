//! Component usage

use core::str;

use rice_dom::{DOM, StyleSheet};
use rice_layout::{Align, Direction, Layout};
use tree_sitter::Node;

use crate::properties::parse_property;

pub fn parse_component(node: Node, content: &[u8], dom: &mut DOM) -> usize {
    let mut layout = Layout::default();
    let mut stylesheet = StyleSheet::default();
    let mut children = Vec::new();

    for child in node.named_children(&mut node.walk()) {
        // TODO: check for error node
        match child.kind() {
            // Classname: set some default properties based on the component type
            "classname" => {
                let name = str::from_utf8(&content[child.byte_range()]).unwrap();
                match name {
                    "Column" => {
                        layout.direction = Direction::Vertical(Align::default());
                    }
                    "Row" => {
                        layout.direction = Direction::Horizontal(Align::default());
                    }
                    "Rect" => {}
                    _ => {
                        // TODO: propagate an error instead
                        panic!("Unknown component class name: {}", name);
                    }
                }
            }
            "property" => parse_property(child, content, &mut layout, &mut stylesheet),
            "component" => children.push(parse_component(child, content, dom)),

            // Ignore the rest
            _ => {}
        }
    }

    dom.insert_with_children(layout, stylesheet, children)
}
