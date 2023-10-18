use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

pub struct Row {
    string: String,
    len: usize,
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);

        let mut result = String::new();
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end-start) {
                if grapheme == "\t" {
                    result.push_str("  ");
                } else {
                    result.push_str(grapheme);
                }
            }
        result
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn update_len(&mut self) {
        let graphemes = self.string[..].graphemes(true);
        self.len = graphemes.clone().count() + graphemes.filter(|&grapheme| grapheme == "\t").count() * (2 - 1); // 2-1 because we use a tab size of two for now 
    }
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut result = Self { 
            string: String::from(slice), 
            len: 0,
        };
        result.update_len();
        result
    }
}