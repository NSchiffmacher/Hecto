use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
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

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
        } else {
            let mut result: String = self.string.graphemes(true).take(at).collect();
            let reminder: String = self.string.graphemes(true).skip(at).collect();
            result.push(c);
            result.push_str(&reminder);
            self.string = result;
        }
        self.update_len();
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        } else {
            let mut result: String = self.string.graphemes(true).take(at).collect();
            let reminder: String = self.string.graphemes(true).skip(at+1).collect();
            result.push_str(&reminder);
            self.string = result;
        }
        self.update_len();
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.update_len();
    }

    pub fn split(&mut self, at: usize) -> Self {
        let beginning: String = self.string.graphemes(true).take(at).collect();
        let end: String = self.string.graphemes(true).skip(at).collect();

        self.string = beginning;
        self.update_len();

        Self::from(&end[..])
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

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
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