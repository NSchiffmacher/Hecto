use std::cmp;
use unicode_segmentation::UnicodeSegmentation;
use termion::color;

use crate::HighlightingOptions;
use crate::SearchDirection;
use crate::highlighting;

#[derive(Default)]
pub struct Row {
    string: String,
    highlighting: Vec<highlighting::Type>,
    len: usize,
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        
        let mut current_highlighting = &highlighting::Type::None;
        let mut result = format!("{}", color::Fg(current_highlighting.to_color()));
        for (index, grapheme) in self.string[..]
            .graphemes(true)
            .enumerate()
            .skip(start)
            .take(end-start) {
                if let Some(c) = grapheme.chars().next() {
                    let highlighting_type = self
                        .highlighting
                        .get(index)
                        .unwrap_or(&highlighting::Type::None);

                    if highlighting_type != current_highlighting {
                        let start_highlighting = format!("{}", color::Fg(highlighting_type.to_color()));
                        result.push_str(&start_highlighting);
                        current_highlighting = highlighting_type;
                    }

                    if c == '\t' {
                        result.push_str(" ");
                    } else {
                        result.push(c);
                    }
                }
            }
        let end_highlighting = format!("{}", color::Fg(color::Reset));
        result.push_str(&end_highlighting);
        result
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
            self.len += 1;
            return;
        } 
        
        let mut result = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string.graphemes(true).enumerate() {
            length += 1;
            if index == at {
                length += 1;
                result.push(c);
            }
            result.push_str(grapheme);
        }

        self.len = length;
        self.string = result;
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        } 

        let mut result = String::new();
        let mut length = 0;

        for (index, grapheme) in self.string.graphemes(true).enumerate() {
            if index != at {
                length += 1;
                result.push_str(grapheme);
            } 
        }
        
        self.len = length;
        self.string = result;
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len;
    }

    pub fn split(&mut self, at: usize) -> Self {
        let mut row = String::new();
        let mut length = 0;

        let mut splitted_row = String::new();
        let mut splitted_length = 0;

        for (index, grapheme) in self.string.graphemes(true).enumerate() {
            if index < at {
                row.push_str(grapheme);
                length += 1;
            } else {
                splitted_row.push_str(grapheme);
                splitted_length += 1;
            }
        }
        
        self.string = row;
        self.len = length;

        Self {
            string: splitted_row,
            len: splitted_length,
            highlighting: Vec::new(), // TO CHANGE
        }
    }

    pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
        if at > self.len || query.is_empty() {
            return None;
        }

        let start = match direction {
            SearchDirection::Forward => at,
            SearchDirection::Backward => 0,
        };
        let end = match direction {
            SearchDirection::Forward => self.len,
            SearchDirection::Backward => at,
        };
        

        let substring: String = self
            .string
            .graphemes(true)
            .skip(start)
            .take(end-start)
            .collect();

        let matching_byte_index = match direction {
            SearchDirection::Forward => substring.find(query),
            SearchDirection::Backward => substring.rfind(query)
        };
        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in 
                substring.grapheme_indices(true).enumerate() {
                    if matching_byte_index == byte_index {
                        return Some(start + grapheme_index);
                    }
            }
        }

        None
    }

    pub fn highlight(&mut self, opts: &HighlightingOptions, word: Option<&str>) {
        let mut highlightings = Vec::new();
        let chars: Vec<_> = self.string.chars().collect();
        let mut matches = Vec::new();
        let mut search_index = 0;

        // Search matches finding
        if let Some(word) = word {
            while let Some(search_match) = self.find(word, search_index, SearchDirection::Forward) {
                matches.push(search_match);
                if let Some(next_index) = search_match.checked_add(word[..].graphemes(true).count()) {
                    search_index = next_index;
                } else {
                    break;
                }
            }
        }

        let mut index = 0;
        let mut in_string = false;
        let mut prev_seperator = true;
        let mut prev_highlighting;
        'main_loop: while let Some(c) = chars.get(index) {
            prev_highlighting = highlightings.get(index.saturating_sub(1)).unwrap_or(&highlighting::Type::None);

            // Search results highlighting
            if let Some(word) = word {
                if matches.contains(&index) {
                    for _ in word[..].graphemes(true) {
                        index += 1;
                        highlightings.push(highlighting::Type::SearchMatch);
                    }
                    continue;
                }
            }

            // Strings highlighting
            if opts.strings() {
                if in_string || *c == '"' {
                    highlightings.push(highlighting::Type::String);
                    if *c == '"' {
                        in_string = !in_string;
                        prev_seperator = true;
                    } else {
                        prev_seperator = false;
                    }
                    index += 1;
                    continue;
                }
            }

            // Chars highlighting
            if opts.characters() && !in_string && *c == '\'' {
                prev_seperator = true;
                if let Some(next_char) = chars.get(index.saturating_add(1)) {
                    let closing_index = if *next_char == '\\' {
                        index.saturating_add(3)
                    } else {
                        index.saturating_add(2)
                    };
                    if let Some(closing_char) = chars.get(closing_index) {
                        if *closing_char == '\'' {
                            for _ in index..closing_index.saturating_add(1) {
                                highlightings.push(highlighting::Type::Character);
                            }
                            index = closing_index.saturating_add(1);
                            continue;
                        }
                    }
                }
            }

            // Single line comment highlighting
            if *c == '/' {
                if let Some(comment_char) = chars.get(index.saturating_add(1)) {
                    if *comment_char == '/' {
                        // Rest of the line is a char
                        for _ in index..chars.len() {
                            highlightings.push(highlighting::Type::Comment);
                        }
                        break;
                    }
                }
            }

            // Primary keywords 
            let first_char = index == 0;
            if first_char || c.is_whitespace() || (c.is_ascii_punctuation() && *c != '_'){
                let mut primary_keyword_found;
                if !first_char {
                    index += 1;
                } 
                for word in opts.primary_keywords() {
                    primary_keyword_found = true;
                    for (keyword_index, keyword_char) in word.chars().enumerate() {
                        if let Some(c) = chars.get(index.saturating_add(keyword_index)) {
                            if *c != keyword_char {
                                primary_keyword_found = false;
                                break;
                            }
                        } else {
                            primary_keyword_found = false;
                            break;
                        }
                    }
                    if let Some(c) = chars.get(index.saturating_add(word.len())) {
                        if c.is_ascii_alphanumeric(){
                            primary_keyword_found = false;
                        }
                    }
                    if primary_keyword_found {
                        if !first_char {
                            highlightings.push(highlighting::Type::None);
                        }
                        for _ in 0..word.len() {
                            highlightings.push(highlighting::Type::PrimaryKeyword);
                            index += 1;
                        }
                        continue 'main_loop;
                    }
                }
                if !first_char {
                    index -= 1;
                }
            }

            // Secondary keywords 
            if first_char || c.is_whitespace() || (c.is_ascii_punctuation() && *c != '_'){
                let mut secondary_keyword_found;
                for word in opts.secondary_keywords() {
                    secondary_keyword_found = true;
                    if !first_char {
                        index += 1;
                    } 
                    for (keyword_index, keyword_char) in word.chars().enumerate() {
                        if let Some(c) = chars.get(index.saturating_add(keyword_index)) {
                            if *c != keyword_char {
                                secondary_keyword_found = false;
                                break;
                            }
                        } else {
                            secondary_keyword_found = false;
                            break;
                        }
                    }
                    if let Some(c) = chars.get(index.saturating_add(word.len())) {
                        if c.is_ascii(){
                            secondary_keyword_found = false;
                        }
                    }
                    if secondary_keyword_found {
                        if !first_char {
                            highlightings.push(highlighting::Type::None);
                        }
                        for _ in 0..word.len() {
                            highlightings.push(highlighting::Type::SecondaryKeyword);
                            index += 1;
                        }
                        continue 'main_loop;
                    }
                    if !first_char {
                        index -= 1;
                    }
                }
            }

            // Numbers highlighting
            if opts.numbers() && (c.is_ascii_digit() && (prev_seperator || (prev_highlighting == &highlighting::Type::Number))) || (prev_highlighting == &highlighting::Type::Number && (c == &'.' || c == &'_')) {
                highlightings.push(highlighting::Type::Number);
            } else {
                highlightings.push(highlighting::Type::None);
            }
            prev_seperator = c.is_ascii_whitespace() || c.is_ascii_punctuation();
            index += 1;
        }

        self.highlighting = highlightings;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            len: slice.graphemes(true).count(),
            highlighting: Vec::new(),
        }
    }
}
