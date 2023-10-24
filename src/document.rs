use crate::Row;
use crate::Position;
use crate::SearchDirection;

use std::fs;
use std::io::{Error, Write};

#[derive(Default)]
pub struct Document {
    pub filename: Option<String>,
    rows: Vec<Row>,
    dirty: bool,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for row_content in content.lines() {
            let mut row = Row::from(row_content);
            row.highlight(None);
            rows.push(row);
        }
        Ok(Self { 
            filename: Some(filename.to_string()),
            rows,
            dirty: false,
         })
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(filename) = &self.filename {
            let mut file = fs::File::create(filename)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }
        Ok(())
    }

    pub fn highlight(&mut self, word: Option<&str>) {
        for row in &mut self.rows {
            row.highlight(word);
        }
    }

    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.len() {
            return;
        } 
        
        self.dirty = true;

        if c == '\n' {
            self.insert_newline(at);
            return;
        }

        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            row.highlight(None);
            self.rows.push(row);
        } else {
            let row = &mut self.rows[at.y];
            row.insert(at.x, c);
            row.highlight(None);
        }
    }

    pub fn insert_newline(&mut self, at: &Position) {
        if at.y == self.len() {
            self.rows.push(Row::default());
            return;
        }

        let current_row = &mut self.rows[at.y];
        let mut new_row = current_row.split(at.x);

        current_row.highlight(None);
        new_row.highlight(None);

        self.rows.insert(at.y + 1, new_row);
    }

    pub fn delete(&mut self, at: &Position) {
        let len = self.len();
        if at.y >= len {
            return;
        }

        self.dirty = true;
        if at.x == self.rows[at.y].len() && at.y < len - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = &mut self.rows[at.y];
            row.highlight(None);
            row.append(&next_row);
        } else {
            let row = &mut self.rows[at.y];
            row.delete(at.x);
            row.highlight(None);
        }
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn find(&self, query: &str, at: &Position, direction: SearchDirection) -> Option<Position> {
        if at.y >= self.rows.len() {
            return None;
        }
        
        let mut position = Position { x: at.x, y: at.y };

        let start = match direction {
            SearchDirection::Forward => at.y,
            SearchDirection::Backward => 0,
        };
        let end = match direction {
            SearchDirection::Forward => self.rows.len(),
            SearchDirection::Backward => at.y.saturating_add(1),
        };

        for _ in start..end {
            if let Some(row) = self.rows.get(position.y) {
                if let Some(x) = row.find(&query, position.x, direction) {
                    position.x = x;
                    return Some(position);
                }
                match direction {
                    SearchDirection::Forward => {
                        position.y = position.y.saturating_add(1);
                        position.x = 0;
                    },
                    SearchDirection::Backward => {
                        position.y = position.y.saturating_sub(1);
                        position.x = self.rows[position.y].len();
                    }
                };
            } else {
                return None;
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}