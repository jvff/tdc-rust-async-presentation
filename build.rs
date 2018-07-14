extern crate comrak;

use std::fs;
use std::path::PathBuf;

use comrak::{markdown_to_html, ComrakOptions};

fn main() {
    let static_dir = PathBuf::from("static");

    if !static_dir.is_dir() {
        fs::create_dir(&static_dir).expect(&format!(
            "Failed to create static dir at: {}",
            static_dir
                .canonicalize()
                .unwrap_or_else(|_| static_dir.clone())
                .display(),
        ));
    }

    let notes_input = fs::read_to_string("notes.md")
        .expect("Failed to read notes.md input file");
    let notes_output =
        markdown_to_html(&notes_input, &ComrakOptions::default());
    let notes_output_file = static_dir.join("notes.html");

    fs::write(&notes_output_file, &notes_output).expect(&format!(
        "Failed to write {} output file",
        notes_output_file.canonicalize().unwrap_or(notes_output_file).display(),
    ));
}
