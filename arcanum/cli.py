"""Arcanum CLI — unified interface to the domain intelligence suite."""

from __future__ import annotations

import argparse
import sys

from arcanum import __version__


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="arcanum",
        description="The complete domain intelligence suite",
    )
    parser.add_argument(
        "-V", "--version",
        action="version",
        version=f"%(prog)s {__version__}",
    )

    sub = parser.add_subparsers(dest="command")

    # Placeholder sub-commands — each will delegate to the respective tool.
    # Dispatch notes:
    #   seer / tome  — installs PyO3 bindings only; CLI dispatch must locate
    #                  the Rust binary via shutil.which("seer") / shutil.which("tome")
    #   tower        — entry point is `tower-mcp`, not `tower`
    #   scrolls      — no CLI exists; subcommand must use the Python API directly
    #   familiar     — requires Python 3.11+
    sub.add_parser("seer", help="Domain diagnostics (WHOIS, RDAP, DNS, status)")
    sub.add_parser("tome", help="Domain knowledge base (TLDs, record types, glossary)")
    sub.add_parser("tower", help="Unified MCP server for Seer and Tome")
    sub.add_parser("scrolls", help="AI agent skill definitions and reference docs")
    sub.add_parser("familiar", help="Conversational AI agent for domain intelligence")

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    if args.command is None:
        parser.print_help()
        return 0

    # TODO: dispatch to sub-project CLIs
    print(f"arcanum {args.command}: not yet implemented")
    return 1


def _entry_point():
    """Console-script entry point that propagates return codes to the shell."""
    sys.exit(main())


if __name__ == "__main__":
    _entry_point()
