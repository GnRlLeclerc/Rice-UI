use std::{
    fs,
    io::{self},
    path::Path,
};

use clap::Parser;

use crate::root::format_source_file;

mod components;
mod enums;
mod properties;
mod root;
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
        format_source_file(tree, content.as_bytes(), &mut io::stdout()).unwrap();
    }
}
