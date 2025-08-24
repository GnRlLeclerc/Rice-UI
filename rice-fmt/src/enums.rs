//! Format enum declarations

use std::io;

use tree_sitter::Node;

use crate::utils::{format_indent, format_lines};

pub fn format_enum_decl<W: io::Write>(
    node: Node,
    depth: usize,
    content: &[u8],
    writer: &mut W,
) -> io::Result<()> {
    for child in node.named_children(&mut node.walk()) {
        match child.kind() {
            "docstring" => format_lines(child, depth, content, writer)?,
            "classname" => {
                format_indent(depth, writer)?;
                writer.write_all(b"enum ")?;
                writer.write_all(&content[child.byte_range()])?;
                writer.write_all(b" {\n")?;
            }
            "enum_variant_decl" => {
                format_enum_variant_decl(child, depth + 1, content, writer)?;
            }
            _ => unreachable!("Unknown enum decl child node type: {}", child.kind()),
        }
    }
    format_indent(depth, writer)?;
    writer.write_all(b"}\n")
}

pub fn format_enum_variant_decl<W: io::Write>(
    node: Node,
    depth: usize,
    content: &[u8],
    writer: &mut W,
) -> io::Result<()> {
    for child in node.named_children(&mut node.walk()) {
        match child.kind() {
            "docstring" => format_lines(child, depth, content, writer)?,
            "identifier" => {
                format_indent(depth, writer)?;
                writer.write_all(&content[child.byte_range()])?;
                writer.write_all(b"\n")?;
            }
            _ => unreachable!(
                "Unknown enum variant decl child node type: {}",
                child.kind()
            ),
        };
    }
    Ok(())
}
