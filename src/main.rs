// save as del_empty_pngs_fast.rs
use image::GenericImageView;
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn is_fully_transparent(path: &Path) -> bool {
    if let Ok(img) = image::open(path) {
        return img.pixels().all(|(_, _, pixel)| pixel[3] == 0);
    }
    false
}

fn main() {
    let root = std::env::args()
        .nth(1)
        .expect("Usage: del_empty_pngs_fast <dir>");
    
    let paths: Vec<_> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && e.path()
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s.eq_ignore_ascii_case("png"))
                    .unwrap_or(false)
        })
        .collect();
    
    paths.par_iter().for_each(|entry| {
        let path = entry.path();
        if is_fully_transparent(path) {
            if let Err(e) = fs::remove_file(path) {
                eprintln!("Failed to delete {:?}: {}", path, e);
            }
        }
    });
}