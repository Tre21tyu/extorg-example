# extorg - Extension-based File Organizer

A simple Rust CLI tool that automatically organizes files in the current directory into categorized subdirectories based on file extensions.

## What it does

`extorg` scans the current directory for files and moves them into an `assets/` directory, organized by file type:

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
1. Create an `assets/` directory (if it doesn't exist)
2. Create subdirectories for each file type
3. Scan for files matching the supported extensions
4. Show you what will be moved
5. Move the files to their appropriate directories

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

Step 2: Scanning for files to organize...
Found 5 file(s) to organize:

  photo.jpg -> assets/images/
  report.pdf -> assets/documents/
  data.csv -> assets/datasets/
  notes.md -> assets/markdown_files/
  song.mp3 -> assets/sounds/

Step 3: Moving files...
Moved: /Users/you/Downloads/photo.jpg -> assets/images/photo.jpg
Moved: /Users/you/Downloads/report.pdf -> assets/documents/report.pdf
Moved: /Users/you/Downloads/data.csv -> assets/datasets/data.csv
Moved: /Users/you/Downloads/notes.md -> assets/markdown_files/notes.md
Moved: /Users/you/Downloads/song.mp3 -> assets/sounds/song.mp3

=== Summary ===
Successfully moved: 5 file(s)

Done!
```

## Important Notes

- **Files are MOVED, not copied** - the original files will be relocated
- Only files with recognized extensions are moved
- Existing `assets/` directory and subdirectories are reused if they exist
- The program operates on the current working directory

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
