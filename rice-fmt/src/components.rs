//! Component declaration and usage

use std::io;

use anyhow::Result;
use tree_sitter::Node;

use crate::{
    properties::{format_property, format_property_decl},
    utils::{format_indent, format_lines, node_error},
};

pub fn format_component_decl<W: io::Write>(
    node: Node,
    depth: usize,
    content: &[u8],
    writer: &mut W,
) -> Result<()> {
    for child in node.named_children(&mut node.walk()) {
        match child.kind() {
            "docstring" => format_lines(child, depth, content, writer)?,
            "classname" => {
                format_indent(depth, writer)?;
                writer.write_all(b"component ")?;
                writer.write_all(&content[child.byte_range()])?;
                writer.write_all(b" {\n")?;
            }
            "property_decl" => format_property_decl(child, depth + 1, content, writer)?,
            "comment" => format_lines(child, depth + 1, content, writer)?,
            "component" => format_component(child, depth + 1, content, writer)?,
            _ => node_error(node, content)?,
        };
    }
    format_indent(depth, writer)?;
    writer.write_all(b"}\n")?;
    Ok(())
}

pub fn format_component<W: io::Write>(
    node: Node,
    depth: usize,
    content: &[u8],
    writer: &mut W,
) -> Result<()> {
    for child in node.named_children(&mut node.walk()) {
        match child.kind() {
            "docstring" => format_lines(child, depth, content, writer)?,
            "classname" => {
                format_indent(depth, writer)?;
                writer.write_all(&content[child.byte_range()])?;
                writer.write_all(b" {\n")?;
            }
            "property" => format_property(child, depth + 1, content, writer)?,
            "comment" => format_lines(child, depth + 1, content, writer)?,
            "component" => format_component(child, depth + 1, content, writer)?,
            _ => node_error(node, content)?,
        };
    }
    format_indent(depth, writer)?;
    writer.write_all(b"}\n")?;
    Ok(())
}
