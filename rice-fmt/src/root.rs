//! Format root nodes & source file

use std::io;

use tree_sitter::{Node, Tree};

use crate::{
    components::{format_component, format_component_decl},
    enums::format_enum_decl,
};

pub fn format_source_file<W: io::Write>(
    tree: Tree,
    content: &[u8],
    writer: &mut W,
) -> io::Result<()> {
    for (i, node) in tree.root_node().children(&mut tree.walk()).enumerate() {
        if i > 0 {
            writer.write_all(b"\n").unwrap();
        }
        format_root_node(node, &content, writer).unwrap();
    }
    Ok(())
}

pub fn format_root_node<W: io::Write>(
    node: Node,
    content: &[u8],
    writer: &mut W,
) -> io::Result<()> {
    match node.kind() {
        "enum_decl" => format_enum_decl(node, 0, content, writer)?,
        "component_decl" => format_component_decl(node, 0, content, writer)?,
        "component" => format_component(node, 0, content, writer)?,
        _ => unreachable!("Unknown root node type: {}", node.kind()),
    };

    Ok(())
}
