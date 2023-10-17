mod editor;
mod terminal;
mod document;

pub use document::Document;

pub use editor::Editor;
pub use editor::Position;

pub use terminal::Terminal;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
