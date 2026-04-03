# CLAUDE.md - Arcanum

Arcanum is the meta-package and unified CLI for the domain intelligence suite. It bundles Seer, Tome, Tower, Scrolls, and Familiar under a single install with optional dependency groups.

**Status:** Early development — CLI dispatcher is stubbed; TUI skeleton in place.

---

## Structure

```
arcanum/
├── Cargo.toml              # Rust workspace root
├── rustfmt.toml            # Rust formatting config
├── pyproject.toml           # Python package config with optional deps and uv sources
├── arcanum/                 # Python package
│   ├── __init__.py          # Version: 0.1.0
│   └── cli.py               # Unified CLI dispatcher (argparse)
└── arcanum-tui/             # Rust TUI binary (Ratatui)
    ├── Cargo.toml
    └── src/
        ├── main.rs          # Entry point, terminal lifecycle, panic hook
        ├── app.rs           # App state machine (Tab enum, dispatch)
        ├── event.rs         # Crossterm event loop + tick timer
        ├── action.rs        # Action enum (user intents)
        ├── error.rs         # ArcanumError type
        └── ui/
            ├── mod.rs       # Root render, 3-zone layout
            ├── theme.rs     # Catppuccin Frappe palette + style helpers
            ├── header.rs    # Title + tab bar
            ├── footer.rs    # Keybinding hints
            ├── dashboard.rs # Suite overview landing page
            ├── seer.rs      # Domain diagnostics tab (placeholder)
            └── tome.rs      # Reference database tab (placeholder)
```

---

## CLI (cli.py)

Entry point: `arcanum` (console script calls `arcanum.cli:_entry_point`, which wraps `main()` with `sys.exit()`)

```
arcanum seer       # Domain diagnostics (WHOIS, RDAP, DNS, status)
arcanum tome       # Knowledge base (TLDs, record types, glossary)
arcanum tower      # Unified MCP server
arcanum scrolls    # AI agent skill definitions and reference docs
arcanum familiar   # Conversational AI agent
arcanum -V         # Version
```

**Current state:** All subcommands print `"arcanum {command}: not yet implemented"` and exit with code 1. The dispatcher needs to delegate to each sub-project's CLI.

**Dispatch notes:**
- `seer` / `tome` — pip extras install PyO3 bindings only; dispatch must locate the Rust CLI binary via `shutil.which`
- `tower` — the actual entry point is `tower-mcp`, not `tower`
- `scrolls` — has no CLI; the subcommand must use the Python API directly
- `familiar` — requires Python 3.11+ (only component with that constraint)

Uses `argparse` (not clap/click) — intentionally minimal.

---

## Installation

```bash
# Full suite
pip install arcanum

# Individual components
pip install arcanum[seer]
pip install arcanum[tome]
pip install arcanum[tower]
pip install arcanum[scrolls]
pip install arcanum[familiar]
```

### Local Development with UV

`pyproject.toml` includes `[tool.uv.sources]` pointing to sibling directories:

```toml
[tool.uv.sources]
seer = { path = "../seer/seer-py", editable = true }
tome = { path = "../tome/tome-py", editable = true }
tower = { path = "../tower", editable = true }
scrolls = { path = "../scrolls", editable = true }
familiar = { path = "../familiar", editable = true }
```

This allows `uv pip install -e .` to resolve all siblings for local development.

---

## Dependencies

**Required (full install):**
- `seer>=0.13.0`
- `tome>=0.1.0`
- `tower>=0.1.0`
- `scrolls>=0.1.0`
- `familiar>=0.1.0`

Each is also available as an optional dependency group for selective install.

**Dev:** `pytest>=7.0`, `pytest-asyncio>=0.21`

**Requires Python 3.9+** (familiar extra requires 3.11+)

---

## Build

### Python (CLI dispatcher)

```bash
pip install -e .         # or: uv pip install -e .
arcanum --version        # Verify install
```

Build backend: Hatchling.

### Rust (TUI)

```bash
cargo build --release
cargo clippy -- -D warnings
cargo fmt --check
./target/release/arcanum-tui   # Launch TUI, q to quit
```

Binary name: `arcanum-tui` (distinct from the Python `arcanum` console script).

**Dependencies:** seer-core and tome-core via path deps to sibling repos (`../seer/seer-core`, `../tome/tome-core`). Uses ratatui 0.29 + crossterm 0.28 (matching seer/tome versions).

---

## TUI Architecture (arcanum-tui)

The TUI is a pure presentation layer — all business logic lives in seer-core and tome-core.

**Key modules:**
- `app.rs` — Central state: `App` struct with `Tab` enum (Dashboard/Seer/Tome), key→action mapping, dispatch
- `event.rs` — Spawns a tokio task that polls crossterm events + emits ticks at 4Hz via `mpsc::UnboundedChannel`
- `action.rs` — `Action` enum decouples user intents from key bindings
- `error.rs` — `ArcanumError` with `From` impls for `SeerError` and `TomeError`
- `ui/theme.rs` — Full Catppuccin Frappe palette as `Color::Rgb` constants + style helpers
- `ui/mod.rs` — Root render splits into header (3 lines), body (fills), footer (1 line)

**Current state:** Tab switching works (Tab/Shift-Tab/1-3), q quits. Seer and Tome tabs render placeholder layouts. No async operations or text input wired yet.

**Patterns:**
- Follows same workspace conventions as seer/tome (workspace deps, lints, rustfmt)
- Panic hook restores terminal before printing
- Event loop runs in a spawned tokio task
- All rendering takes `&App` (immutable) — state mutation happens only in `dispatch()`
