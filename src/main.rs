use clap::Parser;
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

const SNIPPET_SUFFIX: &str = ".snippet";

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
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
