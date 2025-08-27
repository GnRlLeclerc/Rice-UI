use std::{fs, io, path::Path};

use anyhow::{Context, Result};
use clap::Parser;
use walkdir::WalkDir;

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

    /// Check formatting
    #[arg(short, long, default_value_t = false)]
    check: bool,

    /// Format in place
    #[arg(short, long, default_value_t = false)]
    inplace: bool,
}

fn main() {
    let args = Args::parse();
    let mut success = true;

    if Path::new(&args.path).is_dir() {
        for entry in WalkDir::new(&args.path).into_iter().filter_map(|e| e.ok()) {
            // Filter out non .rice files
            match entry.file_name().to_str() {
                Some(name) => {
                    if !name.ends_with(".rice") {
                        continue;
                    }
                }
                None => continue,
            }

            success = match format_file(entry.path(), args.inplace, args.check) {
                Ok(s) => success && s,
                Err(e) => {
                    eprintln!("Error formatting file {}: {}", entry.path().display(), e);
                    eprintln!("Aborting.");
                    std::process::exit(1);
                }
            };
        }
    } else {
        success = match format_file(&args.path, args.inplace, args.check) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error formatting file {}: {}", &args.path, e);
                std::process::exit(1);
            }
        };
    }

    if args.check && !success {
        eprintln!("Improper formatting for path {}", &args.path);
        std::process::exit(1);
    }
}

pub fn format_file<P: AsRef<Path>>(path: P, inplace: bool, check: bool) -> Result<bool> {
    // Parse the file
    let content = fs::read_to_string(&path)
        .context(format!("Failed to read file: {}", &path.as_ref().display()))?;

    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&rice_grammar::LANGUAGE.into())
        .context("Error loading Rice grammar")?;

    let tree = parser
        .parse(&content, None)
        .context("Error parsing content with Rice grammar")?;

    // Format it with the given options
    if inplace {
        let mut file = fs::File::create(&path).context(format!(
            "Failed to open file for writing: {}",
            &path.as_ref().display()
        ))?;
        format_source_file(tree, content.as_bytes(), &mut file)?;
    } else if check {
        let mut buffer = Vec::new();
        format_source_file(tree, content.as_bytes(), &mut buffer)?;
        let formatted = String::from_utf8(buffer).expect("Invalid UTF-8");
        if formatted.cmp(&content) != std::cmp::Ordering::Equal {
            return Ok(false);
        }
    } else {
        format_source_file(tree, content.as_bytes(), &mut io::stdout())?;
    }

    Ok(true)
}
