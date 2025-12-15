# extorg - Extension-based File Organizer

A powerful Rust CLI tool that recursively organizes files throughout your directory tree into a centralized, categorized `assets/` folder based on file extensions.

## What it does

`extorg` **recursively scans** all subdirectories for files and moves them into a centralized `assets/` directory, organized by file type:

- **animation/** - `.webm`, `.gif`
- **canvae/** - `.canvas`
- **datasets/** - `.csv`
- **documents/** - `.pdf`, `.docx`, `.odt`
- **emacs_org_files/** - `.org`
- **html_pages/** - `.html`
- **images/** - `.svg`, `.png`, `.jpg`, `.jpeg`, `.psd`
- **json/** - `.json`
- **markdown_files/** - `.md`
- **obsidian_canvae/** - `.canvas`
- **plaintext_files/** - `.txt`
- **sounds/** - `.mp3`, `.m4a`, `.wav`
- **video/** - `.mp4`, `.mkv`
- **xcalidrawings/** - `.excalidraw`

## Installation

### Option 1: Build from source

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/extorg-example.git
cd extorg-example

# Build in release mode (optimized)
cargo build --release

# The binary will be at ./target/release/extorg-example
```

### Option 2: Install with cargo

```bash
# Install directly from the repository
cargo install --git https://github.com/YOUR_USERNAME/extorg-example.git

# Or if you've cloned it locally
cargo install --path .
```

## Usage

1. Navigate to the directory you want to organize:
   ```bash
   cd ~/Downloads
   ```

2. Run extorg:
   ```bash
   extorg-example
   ```

The program will:
1. Create an `assets/` directory in the current location (if it doesn't exist)
2. Create subdirectories for each file type
3. **Find and merge** any existing `assets/` directories found in subdirectories
4. **Recursively scan** all subdirectories for files matching the supported extensions
5. Show you what will be moved
6. Move the files to their appropriate directories in the main `assets/` folder

## Example Output

```
=== extorg - Extension-based File Organizer ===

Working directory: /Users/you/Downloads

Step 1: Creating directory structure...
Created directory: assets/
  Created subdirectory: assets/animation/
  Created subdirectory: assets/images/
  Created subdirectory: assets/documents/
  ...

Step 2: Searching for existing assets directories...
Found 1 additional assets directory to merge:
Merging: /Users/you/Downloads/project1/assets
  Merged: /Users/you/Downloads/project1/assets/images/logo.png -> assets/images/logo.png
  Removed empty directory: /Users/you/Downloads/project1/assets
Merged 1 file(s) from other assets directories.

Step 3: Scanning all subdirectories for files to organize...
Found 8 file(s) to organize:

  photo.jpg -> assets/images/
  report.pdf -> assets/documents/
  data.csv -> assets/datasets/
  notes.md -> assets/markdown_files/
  song.mp3 -> assets/sounds/
  project1/screenshot.png -> assets/images/
  project2/data/info.json -> assets/json/
  docs/guide.pdf -> assets/documents/

Step 4: Moving files...
Moved: photo.jpg -> assets/images/photo.jpg
Moved: report.pdf -> assets/documents/report.pdf
Moved: data.csv -> assets/datasets/data.csv
Moved: notes.md -> assets/markdown_files/notes.md
Moved: song.mp3 -> assets/sounds/song.mp3
Moved: project1/screenshot.png -> assets/images/screenshot.png
Moved: project2/data/info.json -> assets/json/info.json
Moved: docs/guide.pdf -> assets/documents/guide.pdf

=== Summary ===
Successfully moved: 8 file(s)

Done!
```

## Key Features

### Recursive Search
The program searches through **all subdirectories** to find files matching the configured extensions. No matter how deep a file is nested, it will be found and organized.

### Assets Folder Merging
If multiple `assets/` directories exist in subdirectories, they are automatically merged into the main `assets/` folder at the root level. After merging, empty subdirectory assets folders are removed.

### File Conflict Resolution
If a file with the same name already exists in the destination, the program automatically appends a number to avoid overwriting (e.g., `photo.jpg`, `photo_1.jpg`, `photo_2.jpg`).

## Important Notes

- **Files are MOVED, not copied** - the original files will be relocated
- Only files with recognized extensions are moved
- The program **recursively searches all subdirectories**
- Multiple `assets/` directories are automatically merged into one
- File naming conflicts are handled automatically
- The main `assets/` directory is created in the current working directory

## Building

```bash
# Development build (faster compile, slower runtime)
cargo build

# Release build (slower compile, optimized runtime)
cargo build --release

# Run without building a binary
cargo run

# Run tests
cargo test
```

## Requirements

- Rust 1.70 or higher
- No external dependencies (uses only Rust standard library)

## License

MIT

## Contributing

Pull requests welcome!
