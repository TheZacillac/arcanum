# CLAUDE.md - Arcanum

Arcanum is the meta-package and unified CLI for the domain intelligence suite. It bundles Seer, Tome, Tower, Scrolls, and Familiar under a single install with optional dependency groups.

**Status:** Early development — CLI dispatcher is stubbed, not yet wired to sub-project CLIs.

---

## Structure

```
arcanum/
├── pyproject.toml          # Package config with optional deps and uv sources
└── arcanum/
    ├── __init__.py          # Version: 0.1.0
    └── cli.py               # Unified CLI dispatcher (argparse)
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

```bash
pip install -e .         # or: uv pip install -e .
arcanum --version        # Verify install
```

Build backend: Hatchling.
