use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Creates the assets directory and all subdirectories for organizing files
fn create_directories(base_path: &Path) -> io::Result<()> {
    // Create the main assets directory
    let assets_dir = base_path.join("assets");

    // Check if assets directory already exists
    if !assets_dir.exists() {
        fs::create_dir(&assets_dir)?;
        println!("Created directory: assets/");
    } else {
        println!("Directory already exists: assets/");
    }

    // List of all subdirectories to create
    let subdirs = vec![
        "animation",
        "canvae",
        "datasets",
        "documents",
        "emacs_org_files",
        "html_pages",
        "images",
        "json",
        "markdown_files",
        "obsidian_canvae",
        "plaintext_files",
        "sounds",
        "video",
        "xcalidrawings",
    ];

    // Create each subdirectory
    for subdir in subdirs {
        let dir_path = assets_dir.join(subdir);
        if !dir_path.exists() {
            fs::create_dir(&dir_path)?;
            println!("  Created subdirectory: assets/{}/", subdir);
        } else {
            println!("  Subdirectory already exists: assets/{}/", subdir);
        }
    }

    Ok(())
}

/// Returns a HashMap mapping file extensions to directory names
fn get_extension_mapping() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("webm", "animation"),
        ("gif", "animation"),
        ("canvas", "canvae"),
        ("csv", "datasets"),
        ("pdf", "documents"),
        ("docx", "documents"),
        ("odt", "documents"),
        ("org", "emacs_org_files"),
        ("html", "html_pages"),
        ("svg", "images"),
        ("png", "images"),
        ("jpg", "images"),
        ("jpeg", "images"),
        ("psd", "images"),
        ("json", "json"),
        ("md", "markdown_files"),
        ("canvas", "obsidian_canvae"),
        ("mp3", "sounds"),
        ("m4a", "sounds"),
        ("wav", "sounds"),
        ("txt", "plaintext_files"),
        ("mp4", "video"),
        ("mkv", "video"),
        ("excalidraw", "xcalidrawings"),
    ])
}

/// Finds and categorizes files in the current directory
fn find_and_categorize_files(base_path: &Path) -> io::Result<Vec<(PathBuf, String)>> {
    let mut files_to_move = Vec::new();
    let extension_map = get_extension_mapping();

    // Read all entries in the current directory
    for entry in fs::read_dir(base_path)? {
        let entry = entry?;
        let path = entry.path();

        // Only process files (not directories)
        if path.is_file() {
            // Get the file extension
            if let Some(extension) = path.extension() {
                let ext_str = extension.to_string_lossy().to_lowercase();

                // Check if this extension is in our mapping
                if let Some(target_dir) = extension_map.get(ext_str.as_str()) {
                    files_to_move.push((path, target_dir.to_string()));
                }
            }
        }
    }

    Ok(files_to_move)
}

/// Moves a file to the specified target directory
fn move_file(file_path: &Path, target_dir: &str, base_path: &Path) -> io::Result<()> {
    let assets_dir = base_path.join("assets").join(target_dir);

    // Get the filename
    let file_name = file_path.file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file path"))?;

    // Create the destination path
    let destination = assets_dir.join(file_name);

    // Move the file (rename in Rust moves the file)
    fs::rename(file_path, &destination)?;

    println!("Moved: {} -> assets/{}/{}",
             file_path.display(),
             target_dir,
             file_name.to_string_lossy());

    Ok(())
}

fn main() -> io::Result<()> {
    println!("=== extorg - Extension-based File Organizer ===\n");

    // Get the current working directory
    let current_dir = std::env::current_dir()?;
    println!("Working directory: {}\n", current_dir.display());

    // Step 1: Create directory structure
    println!("Step 1: Creating directory structure...");
    create_directories(&current_dir)?;
    println!();

    // Step 2: Find and categorize files
    println!("Step 2: Scanning for files to organize...");
    let files_to_move = find_and_categorize_files(&current_dir)?;

    if files_to_move.is_empty() {
        println!("No files found to organize.");
        return Ok(());
    }

    println!("Found {} file(s) to organize:\n", files_to_move.len());

    // Show what will be moved
    for (file_path, target_dir) in &files_to_move {
        let file_name = file_path.file_name().unwrap().to_string_lossy();
        println!("  {} -> assets/{}/", file_name, target_dir);
    }

    println!("\nStep 3: Moving files...");

    // Step 3: Move files
    let mut moved_count = 0;
    let mut error_count = 0;

    for (file_path, target_dir) in files_to_move {
        match move_file(&file_path, &target_dir, &current_dir) {
            Ok(_) => moved_count += 1,
            Err(e) => {
                eprintln!("Error moving {}: {}", file_path.display(), e);
                error_count += 1;
            }
        }
    }

    // Summary
    println!("\n=== Summary ===");
    println!("Successfully moved: {} file(s)", moved_count);
    if error_count > 0 {
        println!("Errors encountered: {}", error_count);
    }
    println!("\nDone!");

    Ok(())
}
