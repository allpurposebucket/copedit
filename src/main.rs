mod editor;
use editor::Editor;

fn main() {
    let mut editor = Editor::new();
    if let Err(err) = editor.run() {
        println!("Error running the editor: {:?}", err);
    }
}
