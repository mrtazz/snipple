use std::fs;

const SNIPPET_SUFFIX: &str = ".snippet";

pub struct Manager {
    base_dir: String,
}

impl Manager {
    pub fn new(base_dir: String) -> Manager {
        return Manager { base_dir: base_dir };
    }
    pub fn get_snippet(&self, name: &str) -> Result<String, String> {
        match fs::read_to_string(format!("{}/{}", self.base_dir, name)) {
            Ok(contents) => Ok(String::from(contents)),
            Err(e) => Err(format!(
                "should have been able to read file '{}/{}', got error '{}'",
                self.base_dir, name, e
            )),
        }
    }
    pub fn list_all_snippets(&self) -> Result<Vec<String>, String> {
        return self.list_all_snippets_in_directory(self.base_dir.as_str());
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
                                if filename.ends_with(SNIPPET_SUFFIX) {
                                    match filename
                                        .strip_prefix(format!("{}/", self.base_dir).as_str())
                                    {
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
            base_dir: String::from("./fixtures"),
        };
        let snippets = m.list_all_snippets();
        assert_eq!(snippets.unwrap().len(), 2);
    }
    #[test]
    fn test_get_snippet() {
        let m = Manager {
            base_dir: String::from("./fixtures"),
        };
        let snippet = m.get_snippet("hello.snippet");
        assert_eq!(snippet.unwrap(), "Hello");
    }
    #[test]
    fn test_get_snippet_in_subdir() {
        let m = Manager {
            base_dir: String::from("./fixtures"),
        };
        let snippet = m.get_snippet("nested/test.snippet");
        assert_eq!(snippet.unwrap(), "nested");
    }
}
