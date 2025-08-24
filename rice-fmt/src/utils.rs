//! Formatting utilities

use std::io;
use tree_sitter::Node;

/// Write indentation corresponding to the given depth
pub fn format_indent<W: io::Write>(depth: usize, writer: &mut W) -> io::Result<()> {
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
) -> io::Result<()> {
    // Split into consecutive lines to reindent them properly
    for line in content[node.byte_range()].split(|&b| b == b'\n') {
        format_indent(depth, writer)?;
        writer.write_all(line.trim_ascii())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}
