//! Formatting utilities

use anyhow::{Result, anyhow};
use std::io;
use tree_sitter::Node;

/// Write indentation corresponding to the given depth
pub fn format_indent<W: io::Write>(depth: usize, writer: &mut W) -> Result<()> {
    for _ in 0..depth {
        writer.write_all(b"  ")?;
    }
    Ok(())
}

/// Format and reindent lines as-is, trimming whitespaces (docstrings, comments)
pub fn format_lines<W: io::Write>(
    node: Node,
    depth: usize,
    content: &[u8],
    writer: &mut W,
) -> Result<()> {
    // Split into consecutive lines to reindent them properly
    for line in content[node.byte_range()].split(|&b| b == b'\n') {
        format_indent(depth, writer)?;
        writer.write_all(line.trim_ascii())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}

pub fn node_error(node: Node, content: &[u8]) -> Result<()> {
    Err(anyhow!(
        "Syntax error:\n{}",
        String::from_utf8_lossy(&content[node.byte_range()]).to_string()
    ))
}
