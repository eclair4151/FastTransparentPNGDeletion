// save as del_empty_pngs.rs
use image::GenericImageView;
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use walkdir::WalkDir;

fn is_fully_transparent(path: &Path) -> bool {
    if let Ok(img) = image::open(path) {
        return img.pixels().all(|(_, _, pixel)| pixel[3] == 0);
    }
    false
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: del_empty_pngs <dir> [--streaming]");
        eprintln!("  --streaming: Use streaming mode for better performance on large deeply nested directories with millions of folders and files");
        std::process::exit(1);
    }
    
    let root = &args[1];
    let streaming_mode = args.len() > 2 && args[2] == "--streaming";
    
    let total_files = AtomicUsize::new(0);
    let deleted_files = AtomicUsize::new(0);
    
    let start_time = Instant::now();
    
    if streaming_mode {
        println!("Running in streaming mode (walk and delete simultaneously)...");
        
        WalkDir::new(root)
            .into_iter()
            .par_bridge()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_type().is_file()
                    && e.path()
                        .extension()
                        .and_then(|s| s.to_str())
                        .map(|s| s.eq_ignore_ascii_case("png"))
                        .unwrap_or(false)
            })
            .for_each(|entry| {
                let path = entry.path();
                total_files.fetch_add(1, Ordering::Relaxed);
                
                if is_fully_transparent(path) {
                    if let Err(e) = fs::remove_file(path) {
                        eprintln!("Failed to delete {:?}: {}", path, e);
                    } else {
                        deleted_files.fetch_add(1, Ordering::Relaxed);
                    }
                }
            });
    } else {
        println!("Running in precompute mode (collect all paths first)...");
        
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
        
        total_files.store(paths.len(), Ordering::Relaxed);
        
        paths.par_iter().for_each(|entry| {
            let path = entry.path();
            if is_fully_transparent(path) {
                if let Err(e) = fs::remove_file(path) {
                    eprintln!("Failed to delete {:?}: {}", path, e);
                } else {
                    deleted_files.fetch_add(1, Ordering::Relaxed);
                }
            }
        });
    }
    
    let elapsed = start_time.elapsed();
    let total = total_files.load(Ordering::Relaxed);
    let deleted = deleted_files.load(Ordering::Relaxed);
    let percentage = if total > 0 {
        (deleted as f64 / total as f64) * 100.0
    } else {
        0.0
    };
    
    println!("\n=== Statistics ===");
    println!("Total PNG files processed: {}", total);
    println!("Files deleted: {}", deleted);
    println!("Percentage deleted: {:.2}%", percentage);
    println!("Time elapsed: {:.2?}", elapsed);
}