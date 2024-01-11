use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    snippet_suffix: Option<String>,
    base_dir: Option<String>,
}

const DEFAULT_SNIPPET_LOCATION: &str = "~/.snippets";
const DEFAULT_SNIPPET_SUFFIX: &str = ".snippet";
const CONFIG_LOCATIONS: &'static [&'static str] =
    &["~/.config/snipple/config.yaml", "~/.snipple.yaml"];

impl Config {
    pub fn new(filepath: Option<String>) -> Result<Self, String> {
        match filepath {
            Some(f) => {
                let cfg = try_yaml_read(f)?;
                return Ok(cfg);
            }
            None => {
                // No config file was provided so lets check default locations
                for cfg_path in CONFIG_LOCATIONS {
                    let cfg = try_yaml_read(String::from(*cfg_path))?;
                    return Ok(cfg);
                }
                Err(format!("No config file to parse"))
            }
        }
    }
    pub fn get_snippet_suffix(&self) -> String {
        return String::from(
            self.snippet_suffix
                .as_ref()
                .unwrap_or(&String::from(DEFAULT_SNIPPET_SUFFIX)),
        );
    }
    pub fn get_snippet_base_dir(&self) -> String {
        let home = std::env::var("HOME").unwrap();
        let location = String::from(
            self.base_dir
                .as_ref()
                .unwrap_or(&String::from(DEFAULT_SNIPPET_LOCATION)),
        );
        if location.starts_with("~/") {
            // we should be fine to unwrap() here since we already checked for the prefix
            return format!("{}/{}", home, location.strip_prefix("~/").unwrap());
        } else {
            return location;
        }
    }
}

fn try_yaml_read(fpath: String) -> Result<Config, String> {
    let open_file =
        File::open(fpath.clone()).map_err(|e| format!("unable to read file '{}': {}", fpath, e))?;
    let snipple_config: Config = serde_yaml::from_reader(open_file)
        .map_err(|e| format!("unable to parse config file '{}': {}", fpath, e))?;
    return Ok(snipple_config);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_snippet_base_dir() {
        let location = Config::new(Some(String::from("fixtures/simple_config.yaml")))
            .unwrap()
            .get_snippet_base_dir();
        assert_eq!(location, "./fixtures/snippets");
    }
}
