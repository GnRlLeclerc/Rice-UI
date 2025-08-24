use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use clap::Parser;
use tree_sitter::Node;

use crate::{
    components::{format_component, format_component_decl},
    enums::format_enum_decl,
};

mod components;
mod enums;
mod properties;
mod utils;

/// Format Rice files
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File or folder to format
    #[arg(index = 1)]
    path: String,
}

fn main() {
    let args = Args::parse();

    if Path::new(&args.path).is_dir() {
        unimplemented!("format directory")
    } else {
        let content = fs::read_to_string(&args.path)
            .unwrap_or_else(|_| panic!("Failed to read file: {}", args.path));

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&rice_grammar::LANGUAGE.into())
            .expect("Error loading Rice grammar");

        let tree = parser
            .parse(&content, None)
            .expect("Error parsing content with Rice grammar");

        // Iterate through the tree and format it to the output
        let mut stdout = io::stdout();
        for node in tree.root_node().children(&mut tree.walk()) {
            format_root_node(node, &content.as_bytes(), &mut stdout).unwrap();
        }
    }
}

fn format_root_node<W: Write>(node: Node, content: &[u8], writer: &mut W) -> io::Result<()> {
    match node.kind() {
        "enum_decl" => format_enum_decl(node, 0, content, writer)?,
        "component_decl" => format_component_decl(node, 0, content, writer)?,
        "component" => format_component(node, 0, content, writer)?,
        _ => unreachable!("Unknown root node type: {}", node.kind()),
    };

    Ok(())
}
