use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use std::env::VarError;

use clap::{Parser, Subcommand};

mod snipple;

const DEFAULT_SNIPPET_LOCATION: &str = "~/.snippets";
const SNIPPLE_VERSION: Option<&'static str> = option_env!("SNIPPLE_VERSION");

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(disable_version_flag(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Get a specific snippet
    Get {
        /// Path to the snippet to get
        snippet: String,
    },
    /// List all available snippets
    ListSnippets {},
    /// Print version and exit
    Version {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::ListSnippets {}) => {
            let location = get_snippet_base_dir(std::env::var("HOME"), DEFAULT_SNIPPET_LOCATION);
            match location {
                Ok(location) => {
                    let m = snipple::Manager::new(location);
                    let snippets = m.list_all_snippets().unwrap();
                    for snippet in snippets {
                        println!("{}", snippet)
                    }
                }
                Err(_) => {}
            }
        }
        Some(Commands::Get { snippet }) => {
            let location = get_snippet_base_dir(std::env::var("HOME"), DEFAULT_SNIPPET_LOCATION);
            match location {
                Ok(location) => {
                    let m = snipple::Manager::new(location);
                    let snippet = m.get_snippet(snippet);
                    match snippet {
                        Ok(snippet) => {
                            println!("{}", snippet);
                        }
                        Err(e) => {
                            println!("error getting snippet: {}", e.to_string())
                        }
                    }
                }
                Err(_) => {}
            }
        }
        Some(Commands::Version {}) => {
            println!(
                "snipple {} {} compiled on {}",
                SNIPPLE_VERSION.unwrap_or("dev"),
                CURRENT_PLATFORM,
                COMPILED_ON
            )
        }

        None => {
            println!("unknown command")
        }
    }
}

fn get_snippet_base_dir(home: Result<String, VarError>, location: &str) -> Result<String, String> {
    if location.starts_with("~/") {
        match home {
            Ok(home) => {
                // we should be fine to unwrap() here since we already checked for the prefix
                return Ok(format!("{}/{}", home, location.strip_prefix("~/").unwrap()));
            }
            Err(e) => return Err(String::from(e.to_string())),
        }
    } else {
        return Ok(String::from(location));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_snippet_base_dir() {
        let location =
            get_snippet_base_dir(Ok(String::from("/home/mrtazz")), DEFAULT_SNIPPET_LOCATION);
        assert_eq!(location.unwrap(), "/home/mrtazz/.snippets");
    }
}
