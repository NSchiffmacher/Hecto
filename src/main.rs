mod editor;
mod terminal;
mod document;
mod row;

pub use document::Document;

pub use editor::Editor;
pub use editor::Position;

pub use row::Row;

pub use terminal::Terminal;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
