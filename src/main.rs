use std::env::VarError;
use std::fs;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
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
            let location =
                resolve_tilde_in_snippet_location(std::env::var("HOME"), DEFAULT_SNIPPET_LOCATION);
            match location {
                Ok(location) => {
                    let snippets = find_all_snippets_in_directory(location).unwrap();
                    for snippet in snippets {
                        println!("{}", snippet)
                    }
                }
                Err(_) => {}
            }
        }
        None => {}
    }
}

fn resolve_tilde_in_snippet_location(
    home: Result<String, VarError>,
    location: &str,
) -> Result<String, String> {
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

fn find_all_snippets_in_directory(dir: String) -> Result<Vec<String>, String> {
    let mut snippets: Vec<String> = Vec::new();
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        match path {
            Ok(path) => {
                if path.path().is_dir() {
                    let mut this_snippets =
                        find_all_snippets_in_directory(String::from(path.path().to_str().unwrap()));
                    match this_snippets.as_mut() {
                        Ok(this_snippets) => {
                            snippets.append(this_snippets);
                        }
                        Err(e) => {
                            println!(
                                "Failed to get snippets in directory. Error: {}",
                                e.to_string()
                            )
                        }
                    }
                } else {
                    match path.path().to_str() {
                        Some(filename) => {
                            if filename.ends_with(SNIPPET_SUFFIX) {
                                snippets.push(String::from(filename));
                            }
                        }

                        None => {
                            println!("path isn't valid unicode")
                        }
                    }
                }
            }
            //
            Err(e) => {
                println!("encountered error: {}", e.to_string());
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
    fn test_find_all_snippets_in_directory() {
        let snippets = find_all_snippets_in_directory(String::from("./fixtures"));
        assert_eq!(snippets.unwrap().len(), 2);
    }
    #[test]
    fn test_resolve_tilde_in_snippet_location() {
        let location = resolve_tilde_in_snippet_location(
            Ok(String::from("/home/mrtazz")),
            DEFAULT_SNIPPET_LOCATION,
        );
        assert_eq!(location.unwrap(), "/home/mrtazz/.snippets");
    }
}
