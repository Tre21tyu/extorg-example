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

/// Recursively finds all "assets" directories in subdirectories
fn find_assets_directories(base_path: &Path, main_assets: &Path) -> io::Result<Vec<PathBuf>> {
    let mut assets_dirs = Vec::new();

    fn walk_dir(dir: &Path, main_assets: &Path, assets_dirs: &mut Vec<PathBuf>) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // If this is an assets directory (but not the main one), add it
                if path.file_name() == Some(std::ffi::OsStr::new("assets")) && path != main_assets {
                    assets_dirs.push(path.clone());
                } else if path != main_assets {
                    // Recurse into subdirectories (but not into the main assets dir)
                    walk_dir(&path, main_assets, assets_dirs)?;
                }
            }
        }
        Ok(())
    }

    walk_dir(base_path, main_assets, &mut assets_dirs)?;
    Ok(assets_dirs)
}

/// Merges contents of other assets directories into the main assets directory
fn merge_assets_directories(main_assets: &Path, other_assets_dirs: Vec<PathBuf>) -> io::Result<usize> {
    let mut merged_count = 0;

    for assets_dir in other_assets_dirs {
        println!("Merging: {}", assets_dir.display());

        // Walk through all files in this assets directory
        for entry in fs::read_dir(&assets_dir)? {
            let entry = entry?;
            let source_path = entry.path();

            if source_path.is_dir() {
                // This is a subdirectory (like images/, documents/, etc.)
                let subdir_name = source_path.file_name().unwrap();
                let target_subdir = main_assets.join(subdir_name);

                // Create the subdirectory in main assets if it doesn't exist
                if !target_subdir.exists() {
                    fs::create_dir(&target_subdir)?;
                }

                // Move all files from this subdirectory to the main assets subdirectory
                for file_entry in fs::read_dir(&source_path)? {
                    let file_entry = file_entry?;
                    let file_path = file_entry.path();

                    if file_path.is_file() {
                        let file_name = file_path.file_name().unwrap();
                        let destination = target_subdir.join(file_name);

                        // Handle naming conflicts
                        let final_destination = if destination.exists() {
                            get_unique_filename(&destination)
                        } else {
                            destination
                        };

                        fs::rename(&file_path, &final_destination)?;
                        println!("  Merged: {} -> {}", file_path.display(), final_destination.display());
                        merged_count += 1;
                    }
                }

                // Try to remove the now-empty subdirectory
                let _ = fs::remove_dir(&source_path);
            } else if source_path.is_file() {
                // File directly in assets/ (shouldn't normally happen, but handle it)
                let file_name = source_path.file_name().unwrap();
                let destination = main_assets.join(file_name);

                let final_destination = if destination.exists() {
                    get_unique_filename(&destination)
                } else {
                    destination
                };

                fs::rename(&source_path, &final_destination)?;
                println!("  Merged: {} -> {}", source_path.display(), final_destination.display());
                merged_count += 1;
            }
        }

        // Try to remove the now-empty assets directory
        match fs::remove_dir(&assets_dir) {
            Ok(_) => println!("  Removed empty directory: {}", assets_dir.display()),
            Err(_) => println!("  Could not remove directory (may not be empty): {}", assets_dir.display()),
        }
    }

    Ok(merged_count)
}

/// Generates a unique filename by appending a number if the file already exists
fn get_unique_filename(path: &Path) -> PathBuf {
    let parent = path.parent().unwrap();
    let file_stem = path.file_stem().unwrap().to_string_lossy();
    let extension = path.extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();

    let mut counter = 1;
    loop {
        let new_name = if extension.is_empty() {
            format!("{}_{}", file_stem, counter)
        } else {
            format!("{}_{}.{}", file_stem, counter, extension)
        };

        let new_path = parent.join(&new_name);
        if !new_path.exists() {
            return new_path;
        }
        counter += 1;
    }
}

/// Recursively finds and categorizes files in all subdirectories
fn find_and_categorize_files_recursive(base_path: &Path, main_assets: &Path) -> io::Result<Vec<(PathBuf, String)>> {
    let mut files_to_move = Vec::new();
    let extension_map = get_extension_mapping();

    fn walk_dir(
        dir: &Path,
        main_assets: &Path,
        extension_map: &HashMap<&str, &str>,
        files_to_move: &mut Vec<(PathBuf, String)>,
    ) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                // Check if this file should be organized
                if let Some(extension) = path.extension() {
                    let ext_str = extension.to_string_lossy().to_lowercase();

                    if let Some(target_dir) = extension_map.get(ext_str.as_str()) {
                        files_to_move.push((path, target_dir.to_string()));
                    }
                }
            } else if path.is_dir() {
                // Don't recurse into the main assets directory or any assets subdirectories
                let is_assets_related = path == main_assets
                    || path.starts_with(main_assets)
                    || path.file_name() == Some(std::ffi::OsStr::new("assets"));

                if !is_assets_related {
                    walk_dir(&path, main_assets, extension_map, files_to_move)?;
                }
            }
        }
        Ok(())
    }

    walk_dir(base_path, main_assets, &extension_map, &mut files_to_move)?;
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

    // Handle naming conflicts
    let final_destination = if destination.exists() {
        get_unique_filename(&destination)
    } else {
        destination
    };

    // Move the file (rename in Rust moves the file)
    fs::rename(file_path, &final_destination)?;

    let relative_source = file_path.strip_prefix(base_path)
        .unwrap_or(file_path);

    println!("Moved: {} -> assets/{}/{}",
             relative_source.display(),
             target_dir,
             final_destination.file_name().unwrap().to_string_lossy());

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
    let main_assets = current_dir.join("assets");
    println!();

    // Step 2: Find and merge any existing assets directories
    println!("Step 2: Searching for existing assets directories...");
    let other_assets_dirs = find_assets_directories(&current_dir, &main_assets)?;

    if !other_assets_dirs.is_empty() {
        println!("Found {} additional assets director{} to merge:",
                 other_assets_dirs.len(),
                 if other_assets_dirs.len() == 1 { "y" } else { "ies" });
        let merged_count = merge_assets_directories(&main_assets, other_assets_dirs)?;
        println!("Merged {} file(s) from other assets directories.\n", merged_count);
    } else {
        println!("No additional assets directories found.\n");
    }

    // Step 3: Find and categorize files recursively
    println!("Step 3: Scanning all subdirectories for files to organize...");
    let files_to_move = find_and_categorize_files_recursive(&current_dir, &main_assets)?;

    if files_to_move.is_empty() {
        println!("No files found to organize.");
        return Ok(());
    }

    println!("Found {} file(s) to organize:\n", files_to_move.len());

    // Show what will be moved (limit to first 20 for readability)
    let display_limit = 20;
    for (i, (file_path, target_dir)) in files_to_move.iter().enumerate() {
        if i >= display_limit {
            println!("  ... and {} more file(s)", files_to_move.len() - display_limit);
            break;
        }
        let relative_path = file_path.strip_prefix(&current_dir)
            .unwrap_or(file_path);
        println!("  {} -> assets/{}/", relative_path.display(), target_dir);
    }

    println!("\nStep 4: Moving files...");

    // Step 4: Move files
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
