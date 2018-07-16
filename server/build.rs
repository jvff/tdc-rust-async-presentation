extern crate presentrs;

use std::fs;
use std::path::PathBuf;

use presentrs::Notes;

fn main() {
    let static_dir = PathBuf::from("../static");

    if !static_dir.is_dir() {
        fs::create_dir(&static_dir).expect(&format!(
            "Failed to create static dir at: {}",
            static_dir
                .canonicalize()
                .unwrap_or_else(|_| static_dir.clone())
                .display(),
        ));
    }

    Notes::from_markdown("../notes.md")
        .expect("Failed to read notes")
        .animate_steps()
        .expect("Failed to animate slide steps")
        .generate_html(static_dir)
        .expect("Failed to generate notes file");
}
