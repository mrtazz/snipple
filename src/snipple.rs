use std::fs;

use crate::config;

pub struct Manager<'a> {
    config: &'a config::Config,
}

impl Manager<'_> {
    pub fn new(cfg: &config::Config) -> Manager {
        return Manager { config: cfg };
    }
    pub fn get_snippet(&self, name: &str) -> Result<String, String> {
        match fs::read_to_string(format!("{}/{}", self.config.get_snippet_base_dir(), name)) {
            Ok(contents) => Ok(String::from(contents)),
            Err(e) => Err(format!(
                "should have been able to read file '{}/{}', got error '{}'",
                self.config.get_snippet_base_dir(),
                name,
                e
            )),
        }
    }
    pub fn list_all_snippets(&self) -> Result<Vec<String>, String> {
        return self.list_all_snippets_in_directory(self.config.get_snippet_base_dir().as_str());
    }

    fn list_all_snippets_in_directory(&self, path: &str) -> Result<Vec<String>, String> {
        let mut snippets: Vec<String> = Vec::new();
        let paths = fs::read_dir(path).unwrap();

        for path in paths {
            match path {
                Ok(path) => {
                    if path.path().is_dir() {
                        let mut this_snippets =
                            self.list_all_snippets_in_directory(path.path().to_str().unwrap());
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
                                // only add snippets with the right suffix
                                if filename.ends_with(self.config.get_snippet_suffix().as_str()) {
                                    match filename.strip_prefix(
                                        format!("{}/", self.config.get_snippet_base_dir()).as_str(),
                                    ) {
                                        // only add snippets which had the right prefix. Not sure
                                        // if there is ever a case where that could not be true
                                        Some(filename) => snippets.push(filename.to_string()),
                                        None => {}
                                    }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_all_snippets_in_directory() {
        let m = Manager {
            config: &config::Config::new(Some(String::from("fixtures/simple_config.yaml")), false)
                .unwrap(),
        };
        let snippets = m.list_all_snippets();
        assert_eq!(snippets.unwrap().len(), 2);
    }
    #[test]
    fn test_get_snippet() {
        let m = Manager {
            config: &config::Config::new(Some(String::from("fixtures/simple_config.yaml")), false)
                .unwrap(),
        };
        let snippet = m.get_snippet("hello.snippet");
        assert_eq!(snippet.unwrap(), "Hello");
    }
    #[test]
    fn test_get_snippet_in_subdir() {
        let m = Manager {
            config: &config::Config::new(Some(String::from("fixtures/simple_config.yaml")), false)
                .unwrap(),
        };
        let snippet = m.get_snippet("nested/test.snippet");
        assert_eq!(snippet.unwrap(), "nested");
    }
}
