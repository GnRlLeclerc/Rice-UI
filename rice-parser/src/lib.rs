//! Parse Rice source files into a DOM structure

mod component;
mod properties;
mod values;

use rice_dom::DOM;

use crate::component::parse_component;

pub fn parse(content: &str, dom: &mut DOM) -> usize {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&rice_grammar::LANGUAGE.into())
        .expect("Error loading Rice grammar");

    let tree = parser
        .parse(content, None)
        .expect("Failed to parse content");

    let mut root = None;

    for node in tree.root_node().children(&mut tree.walk()) {
        match node.kind() {
            "component" => {
                if root.is_some() {
                    panic!("Multiple root components found");
                }
                root = Some(parse_component(node, content.as_bytes(), dom));
            }
            _ => {}
        }
    }

    root.expect("No root component found")
}
