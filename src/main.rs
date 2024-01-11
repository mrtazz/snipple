use current_platform::{COMPILED_ON, CURRENT_PLATFORM};

use clap::{Parser, Subcommand};
use serde_json;

mod alfred;
mod config;
mod snipple;

const SNIPPLE_VERSION: Option<&'static str> = option_env!("SNIPPLE_VERSION");

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
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
    List {
        /// Run in alfred compatibility mode
        #[arg(long)]
        alfred: bool,
    },
    /// Print version and exit
    Version {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List { alfred }) => {
            let cfg = config::Config::new(None).unwrap();
            let m = snipple::Manager::new(&cfg);
            let snippets = m.list_all_snippets().unwrap();
            if *alfred {
                let mut result = alfred::ListResult { items: Vec::new() };
                for snippet in snippets {
                    result.items.push(alfred::Item {
                        title: snippet.clone(),
                        arg: snippet.clone(),
                    });
                }
                println!("{}", serde_json::to_string(&result).unwrap());
            } else {
                for snippet in snippets {
                    println!("{}", snippet)
                }
            }
        }
        Some(Commands::Get { snippet }) => {
            let cfg = config::Config::new(None).unwrap();
            let m = snipple::Manager::new(&cfg);
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
