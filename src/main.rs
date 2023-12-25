use std::fs;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// list all available snippets
    ListSnippets {},
}

const SNIPPET_SUFFIX: &str = ".snippet";
const DEFAULT_SNIPPET_LOCATION: &str = "~/.snippets";

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::ListSnippets {}) => {
            let snippets =
                find_all_snippets_in_directory(String::from(DEFAULT_SNIPPET_LOCATION)).unwrap();
            for snippet in snippets {
                println!("{}\n", snippet)
            }
        }
        None => {}
    }
}

fn find_all_snippets_in_directory(dir: String) -> Result<Vec<String>, String> {
    let mut snippets: Vec<String> = Vec::new();
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        match path {
            Ok(ok_path) => match ok_path.path().to_str() {
                Some(filename) => {
                    if filename.ends_with(SNIPPET_SUFFIX) {
                        snippets.push(String::from(filename));
                    }
                }

                None => continue,
            },
            Err(_) => {
                continue;
            }
        }
    }

    return Ok(snippets);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_snippets_in_directory() {
        let snippets = find_all_snippets_in_directory(String::from("./fixtures"));
        assert_eq!(snippets.unwrap().len(), 2);
    }
}
