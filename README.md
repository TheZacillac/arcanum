# Arcanum

**The complete domain intelligence suite — Seer, Tome, Tower, Scrolls, and Familiar, unified under a single install.**

Arcanum is the meta-package that brings together the full stack of domain intelligence tools. Install everything at once, or pick the pieces you need. It also provides a unified CLI for interacting with all of them from one place.

---

## The Suite

| Project | Description | Language |
|---------|-------------|----------|
| **[Seer](https://github.com/TheZacillac/seer)** | Domain diagnostics — WHOIS, RDAP, DNS lookups, propagation monitoring, status checks | Rust + Python |
| **[Tome](https://github.com/TheZacillac/tome)** | Domain knowledge base — TLDs, DNS record types, glossary of domain terminology | Rust + Python |
| **[Tower](https://github.com/TheZacillac/tower)** | Unified MCP server exposing Seer and Tome as 22 AI-agent-ready tools | Python |
| **[Scrolls](https://github.com/TheZacillac/scrolls)** | Skill definitions and reference docs for AI agents working with Seer and Tome | Python |
| **[Familiar](https://github.com/TheZacillac/familiar)** | Conversational AI agent for domain intelligence, powered by LangGraph and Ollama | Python |
| **[Oracle](https://github.com/TheZacillac/oracle)** | Training dataset generator — 423 topics, 26 categories for domain expert LLM | Python |
| **[Vessel](https://github.com/TheZacillac/vessel)** | Fine-tuned domain expert LLM (pre-development) | — |

---

## Architecture

```
                         ┌─────────────────────┐
                         │      Arcanum        │
                         │   (meta-package)    │
                         └────────┬────────────┘
                                  │
         ┌────────────────────────┼────────────────────────┐
         │                        │                        │
┌────────▼────────┐      ┌───────▼───────┐      ┌────────▼────────┐
│    Familiar     │      │    Tower      │      │    Scrolls      │
│  (AI agent)     │      │  (MCP server) │      │  (skill defs)   │
└────────┬────────┘      └───────┬───────┘      └─────────────────┘
         │                       │
         └───────────┬───────────┘
                     │
            ┌────────▼────────┐
            │   Seer + Tome   │
            │  (Rust cores +  │
            │  Python libs)   │
            └─────────────────┘

┌─────────────────────┐      ┌─────────────────────┐
│       Oracle        │ ───> │       Vessel        │
│  (dataset gen)      │      │  (fine-tuned LLM)   │
└─────────────────────┘      └─────────────────────┘
```

---

## Installation

### Full Suite

```bash
uv pip install arcanum
```

This installs everything — Seer, Tome, Tower, Scrolls, and Familiar.

### Individual Components

Install only what you need:

```bash
uv pip install arcanum[seer]       # Domain diagnostics only
uv pip install arcanum[tome]       # Knowledge base only
uv pip install arcanum[tower]      # MCP server (includes Seer + Tome)
uv pip install arcanum[scrolls]    # Skill definitions
uv pip install arcanum[familiar]   # AI agent (includes Seer + Tome)
```

Or install the sub-projects directly:

```bash
uv pip install seer
uv pip install tome
```

### Prerequisites

- Python 3.9+ (Familiar requires 3.11+)
- [uv](https://docs.astral.sh/uv/) (recommended) or pip

---

## CLI

Arcanum provides a unified command-line interface:

```bash
arcanum --help
arcanum seer       # Domain diagnostics
arcanum tome       # Knowledge base queries
arcanum tower      # MCP server management
arcanum scrolls    # Skill and reference docs
arcanum familiar   # Conversational agent
```

The individual tools also retain their own CLIs — Arcanum supplements rather than replaces them.

---

## Development

```bash
git clone https://github.com/TheZacillac/arcanum.git
cd arcanum

uv venv
uv pip install -e ".[dev]"
```

Local development uses `[tool.uv.sources]` in `pyproject.toml` to resolve sub-projects from sibling directories:

```
Projects/
├── arcanum/      # This repo
├── seer/
├── tome/
├── tower/
├── scrolls/
└── familiar/
```

### Running Tests

```bash
uv run pytest
```

---

## Project Structure

```
arcanum/
├── Cargo.toml            # Rust workspace root (for arcanum-tui)
├── pyproject.toml        # Package config, dependencies, UV sources
├── arcanum/
│   ├── __init__.py       # Package version
│   ├── cli.py            # Unified CLI entry point (argparse)
│   └── _logging.py       # Structured logging (text/JSON, file rotation, OpenTelemetry)
├── arcanum-tui/          # Terminal UI (Rust, Ratatui + Crossterm)
│   └── src/
│       ├── main.rs       # Entry point, terminal lifecycle
│       ├── app.rs        # App state, tab switching
│       ├── event.rs      # Keyboard/mouse event loop
│       └── ui/           # Rendering (dashboard, seer, tome tabs)
└── .gitignore
```

---

## Technology Stack

| Component | Technology |
|-----------|------------|
| Package manager | uv |
| Build backend | hatchling |
| Core libraries | Rust (via PyO3) |
| MCP server | Python (mcp SDK) |
| AI agent | LangGraph + Ollama |
| Python | 3.11+ |

---

## License

MIT License - Copyright (c) 2026 Zac Roach
