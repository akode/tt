# Copilot Instructions for tt

## Project Overview

A personal CLI tool (`tt`) for terminal convenience tasks including:
- **Paper management**: Extract and store academic papers (arXiv) to Obsidian vault
- **Daily notes**: Generate and parse daily notes with time tracking
- **Pace calculations**: Convert running/walking paces to km/h
- **Recipes**: Recipe management utilities
- **GitHub Copilot token**: Export GitHub Copilot authentication token

## Build, Test, and Development

### Standard Commands
```bash
cargo build              # Build the project
cargo test               # Run all tests
cargo test <test_name>   # Run specific test (e.g., cargo test from_str)
cargo clippy             # Lint with clippy
cargo fmt                # Format code
cargo run -- <subcommand> # Run with subcommands
```

### Installation
```bash
cargo install --path .   # Install locally
```

### Run Examples
```bash
cargo run -- paper https://www.arxiv.org/pdf/1234.5678.pdf
cargo run -- daily
cargo run -- daily --offset -1  # Yesterday's daily note
cargo run -- pace "5:30"
cargo run -- sum-daily 2024-01-15.md
```

## Architecture

### Configuration System
- Uses `confy` crate for TOML-based configuration
- Default config location shown on startup
- Config paths are resolved using `resolve-path` to handle `~` expansions
- Main config struct: `Config` in `src/config.rs` with Obsidian vault paths

### Module Structure
```
main.rs           # Entry point, config loading, command routing
cli.rs            # Clap-based CLI definitions (Commands enum)
papers/           # Paper extraction and storage
  ├── mod.rs      # Main paper storage logic, source determination
  ├── arxiv.rs    # ArXiv-specific extraction (ID parsing from URLs)
  ├── paper_info.rs # Unified paper metadata structure
  └── markdown.rs # Askama templates for paper markdown
config.rs         # Configuration with Obsidian paths
daily.rs          # Daily note creation & time slot parsing (mdast)
pace.rs           # Pace string parsing and conversion
file_handling.rs  # File/URL fetching utilities
env.rs            # Environment variable operations
errors.rs         # Custom error types
recipes.rs        # Recipe management
```

### Template System
- Uses Askama for templating
- Templates located in `templates/` directory
- `daily.md` - Daily note template with date placeholder
- `paper.md` - Paper metadata template for Obsidian

### Key Data Flows

**Paper Workflow**:
1. URL provided via CLI → `store_paper()` in `papers/mod.rs`
2. Source determination (manual or auto-detect from domain)
3. Fetch paper info from source (e.g., `ArxivPaper::from_url()`)
4. Convert to unified `PaperInfo` struct
5. Render Askama template to markdown file in Obsidian papers directory
6. Download PDF to Obsidian attachments directory

**Daily Note Workflow**:
1. `create_daily()` generates markdown from `DailyTemplate`
2. `sum_time_slots()` parses existing daily notes:
   - Extracts YAML front matter (date field)
   - Parses markdown to mdast (markdown AST)
   - Recursively finds time slots matching `HH:MM - HH:MM:` pattern
   - Calculates total hours worked

## Key Conventions

### Error Handling
- Uses `anyhow::Result` for most functions
- Custom errors in `errors.rs` via `thiserror`
- Panics with `expect()` used in main command flow where errors are unrecoverable

### URL and Path Handling
- `url::Url` for parsing and validating URLs
- ArXiv URLs parsed by examining path segments (`/pdf/` or `/abs/`)
- ArXiv ID extraction removes `.pdf` extension when present
- Paths resolved with `resolve_path` crate to handle shell expansions

### Time Parsing in Daily Notes
- Regex: `(?<start>[0-9]{1,2}:[0-5][0-9]) - (?<end>[0-9]{1,2}:[0-5][0-9]):`
- Format: `HH:MM - HH:MM:` (note trailing colon)
- **Known limitation**: Does not handle midnight crossovers (24:00, 00:00)
- Uses `chrono::NaiveTime` for time arithmetic

### Testing
- Tests colocated in modules using `#[cfg(test)]`
- Example test modules: `config.rs`, `pace.rs`
- Run specific module tests: `cargo test config::` or `cargo test pace::`

### Clap CLI Patterns
- Main `Cli` struct with `#[command(version)]` for version display
- Subcommands defined in `Commands` enum
- Uses derive API: `#[derive(Parser, Subcommand)]`
- Optional arguments with `Option<T>` types

### Dependencies of Note
- `askama` - Template rendering
- `clap` - CLI with derive features
- `confy` - Configuration management (TOML)
- `scraper` - HTML parsing (for paper extraction)
- `markdown` - Markdown AST parsing via `mdast`
- `yaml-front-matter` - YAML header parsing
- `ureq` - HTTP requests for fetching papers

## Obsidian Integration

This tool is tightly coupled with Obsidian vault structure:
- Papers stored in configured `obsidian_papers_dir`
- PDFs stored in `obsidian_attachments_dir`
- Filenames generated from paper metadata
- Markdown templates include Obsidian-specific links and formatting
