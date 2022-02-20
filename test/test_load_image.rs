use std::path::{Path, PathBuf};
use image_operations::ImageOperations;
use app::ImageEditor;

fn from_file(path: &str) -> String {
    let p = Path::new(path);

    match File::open(&p).read_to_string() {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    }
}
#[test]
fn test_open_image() {
    //opening
    //saving
    //path
    let mut editor = ImageEditor::default();
    self.image_operations.load_image_from_path(path::Path::new("C:/Users/User/Desktop/ex.png"));
}
