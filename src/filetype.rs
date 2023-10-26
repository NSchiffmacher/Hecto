

pub struct FileType {
    name: String,
    hl_opts: HighlightingOptions,
}

#[derive(Default)]
pub struct HighlightingOptions {
    numbers: bool,
    strings: bool,
    characters: bool,
    comments: bool,
}

impl HighlightingOptions {
    pub fn numbers(&self) -> bool {
        self.numbers
    }

    pub fn strings(&self) -> bool {
        self.strings
    }

    pub fn characters(&self) -> bool {
        self.characters
    }

    pub fn comments(&self) -> bool {
        self.comments
    }
}

impl FileType {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn highlighting_options(&self) -> &HighlightingOptions {
        &self.hl_opts
    }
}

impl From<&str> for FileType {
    fn from(file_name: &str) -> Self {
        if file_name.ends_with(".rs") {
            Self {
                name: "Rust".to_owned(),
                hl_opts: HighlightingOptions { 
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                }
            }
        } else {
            Self::default()
        }
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No file type"),
            hl_opts: HighlightingOptions::default(),
        }
    }
}
