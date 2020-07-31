
pub trait EasyPath {
    fn stringify(&self) -> &str;
    fn stringify_owned(&self) -> String;
}

impl EasyPath for std::path::Path {
    fn stringify(&self) -> &str {
        self.to_str().unwrap()
    }
    fn stringify_owned(&self) -> String {
        self.stringify().to_string()
    }
}

impl EasyPath for std::path::PathBuf {
    fn stringify(&self) -> &str {
        self.to_str().unwrap()
    }
    fn stringify_owned(&self) -> String {
        self.stringify().to_string()
    }
}

impl EasyPath for &std::ffi::OsStr {
    fn stringify(&self) -> &str {
        self.to_str().unwrap()
    }
    fn stringify_owned(&self) -> String {
        self.to_str().unwrap().to_string()
    }
}
