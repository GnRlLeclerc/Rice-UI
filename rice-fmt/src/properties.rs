//! Properties declaration and usage

use std::io;

use anyhow::Result;
use tree_sitter::Node;

use crate::utils::{format_indent, format_lines, node_error};

pub fn format_property_decl<W: io::Write>(
    node: Node,
    depth: usize,
    content: &[u8],
    writer: &mut W,
) -> Result<()> {
    for child in node.named_children(&mut node.walk()) {
        match child.kind() {
            "docstring" => format_lines(child, depth, content, writer)?,
            "propname" => {
                format_indent(depth, writer)?;
                writer.write_all(&content[child.byte_range()])?;
            }
            // Type
            "classname" => {
                writer.write_all(b" ")?;
                writer.write_all(&content[child.byte_range()])?;
            }
            // Default value
            "value" => {
                writer.write_all(b" = ")?;
                writer.write_all(&content[child.byte_range()])?;
            }
            _ => node_error(node, content)?,
        };
    }
    writer.write_all(b"\n")?;
    Ok(())
}

pub fn format_property<W: io::Write>(
    node: Node,
    depth: usize,
    content: &[u8],
    writer: &mut W,
) -> Result<()> {
    for child in node.named_children(&mut node.walk()) {
        match child.kind() {
            "propname" => {
                format_indent(depth, writer)?;
                writer.write_all(&content[child.byte_range()])?;
            }
            // Value
            "value" => {
                writer.write_all(b": ")?;
                writer.write_all(&content[child.byte_range()])?;
            }
            _ => node_error(node, content)?,
        };
    }
    writer.write_all(b"\n")?;
    Ok(())
}
