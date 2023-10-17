mod editor;
mod terminal;

use editor::Editor;

pub use editor::Position;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
