# del_empty_pngs

A fast, multi-threaded tool to recursively delete fully transparent PNG files (commonly empty XYZ tiles in Leaflet map applications).

## Features

- **Fast**: Uses parallel processing across all CPU cores
- **Recursive**: Scans through all subdirectories
- **Safe**: Only deletes PNGs where every pixel has alpha = 0
- **Cross-format**: Handles RGBA, grayscale+alpha, and other PNG formats

## Installation

### Option 1: Use Pre-compiled Binary (M Series Macs only)

Download the `del_empty_pngs` binary from this repository and make it executable:
```bash
chmod +x del_empty_pngs
```

### Option 2: Compile from Source

**Prerequisites:**
- Rust toolchain (install from https://rustup.rs)

**Steps:**

1. Clone this repository:
```bash
   git clone <repository-url>
   cd del_empty_pngs
```

2. Build the release binary:
```bash
   cargo build --release
```

3. The compiled binary will be at `./target/release/del_empty_pngs`

## Usage

Run the tool with a directory path:
```bash
./del_empty_pngs /path/to/your/tiles [--streaming]
```

### Modes

**Default Mode (Precompute):**
- Best for flat directories
- Collects all PNG paths first, then processes in parallel
- Faster for smaller directory structures

```bash
./del_empty_pngs ~/Documents/map_tiles
```

**Streaming Mode:**
- Best for large, deeply nested directories with millions of folders and files
- Walks directories and deletes simultaneously using parallel iteration
- Avoids memory overhead of precomputing file lists
- Use the `--streaming` flag

```bash
./del_empty_pngs ~/Documents/map_tiles --streaming
```

### Output

The tool will:
1. Recursively scan the directory for all PNG files
2. Check each PNG to see if it's fully transparent
3. Delete any fully transparent PNGs
4. Display statistics including:
   - Total PNG files processed
   - Number of files deleted
   - Percentage deleted
   - Time elapsed
