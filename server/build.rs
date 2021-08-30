extern crate presentrs;

use std::fs;
use std::path::{Path, PathBuf};

use presentrs::{Notes, Slides};

fn main() {
    let output_dir = PathBuf::from("../dist");
    let slides_dir = output_dir.join("slides");

    println!("cargo:rerun-if-changed={}", output_dir.display());

    let mut notes =
        Notes::from_markdown("../notes.md").expect("Failed to read notes");

    create_dir(&output_dir);
    create_dir(&slides_dir);

    notes
        .animate_steps()
        .expect("Failed to animate slide steps");
    notes
        .generate_html(output_dir)
        .expect("Failed to generate notes file");

    let mut slides =
        Slides::from_notes(&notes).expect("Failed to generate slides");

    slides
        .load_from("../slides")
        .expect("Failed to load slides");
    slides.write_to(slides_dir).expect("Failed to write slides");
}

fn create_dir<P: AsRef<Path>>(directory: P) {
    let directory = directory.as_ref();

    if !directory.is_dir() {
        fs::create_dir(&directory).expect(&format!(
            "Failed to create directory at: {}",
            directory
                .canonicalize()
                .unwrap_or_else(|_| directory.to_owned())
                .display(),
        ));
    }
}
