use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    snippet_suffix: Option<String>,
    base_dir: Option<String>,
    debug: Option<bool>,
}

const DEFAULT_SNIPPET_LOCATION: &str = "~/.snippets";
const DEFAULT_SNIPPET_SUFFIX: &str = ".snippet";
const CONFIG_LOCATIONS: &'static [&'static str] =
    &["~/.config/snipple/config.yaml", "~/.snipple.yaml"];

impl Config {
    pub fn new(filepath: Option<String>, debug: bool) -> Result<Self, String> {
        match filepath {
            Some(f) => {
                let mut cfg = try_yaml_read(sanitize_tilde_to_home(f.as_str()))?;
                cfg.debug = Some(debug);
                return Ok(cfg);
            }
            None => {
                // No config file was provided so lets check default locations
                for cfg_path in CONFIG_LOCATIONS {
                    let mut cfg = try_yaml_read(sanitize_tilde_to_home(cfg_path))?;
                    cfg.debug = Some(debug);
                    return Ok(cfg);
                }
                if debug {
                    println!("No config file to parse. Using defaults.");
                }
                Ok(Config {
                    snippet_suffix: Some(String::from(DEFAULT_SNIPPET_SUFFIX)),
                    base_dir: Some(String::from(DEFAULT_SNIPPET_LOCATION)),
                    debug: Some(debug),
                })
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
        let location = String::from(
            self.base_dir
                .as_ref()
                .unwrap_or(&String::from(DEFAULT_SNIPPET_LOCATION)),
        );

        return sanitize_tilde_to_home(location.as_str());
    }
}

fn sanitize_tilde_to_home(path: &str) -> String {
    let home = std::env::var("HOME").unwrap();
    if path.starts_with("~/") {
        // we should be fine to unwrap() here since we already checked for the prefix
        return format!("{}/{}", home, path.strip_prefix("~/").unwrap());
    } else {
        return String::from(path);
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
    fn test_get_simple_config() {
        let cfg = Config::new(Some(String::from("fixtures/test_config.yaml")), false).unwrap();
        let location = cfg.get_snippet_base_dir();
        let suffix = cfg.get_snippet_suffix();
        assert_eq!(location, "./fixtures/snippets");
        assert_eq!(suffix, ".snp");
    }
}
