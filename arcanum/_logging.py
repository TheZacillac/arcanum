"""Arcanum suite logging configuration.

Call ``configure_logging("service-name")`` once at process startup.
After that, use ``get_logger(__name__)`` everywhere.

Environment variables
---------------------
ARCANUM_LOG_LEVEL   Log level (DEBUG, INFO, WARNING, ERROR). Default: INFO.
ARCANUM_LOG_FORMAT  "text" (default) or "json".
ARCANUM_LOG_FILE    Set to "1" / "true" / "yes" to enable file output.
ARCANUM_LOG_DIR     Log directory. Default: ~/.arcanum/logs/
"""

from __future__ import annotations

import contextvars
import json
import logging
import logging.handlers
import os
import sys
import threading
import uuid
from pathlib import Path
from typing import Optional

__all__ = [
    "configure_logging",
    "get_logger",
    "set_correlation_id",
    "get_correlation_id",
    "CORRELATION_ID_VAR",
]

_CONFIGURE_LOCK = threading.Lock()
_CONFIGURED = False

CORRELATION_ID_VAR: contextvars.ContextVar[str] = contextvars.ContextVar(
    "arcanum_correlation_id", default=""
)


# ---------------------------------------------------------------------------
# Formatters
# ---------------------------------------------------------------------------

class ArcanumJsonFormatter(logging.Formatter):
    """Structured JSON formatter matching the Rust tracing-subscriber JSON layout."""

    def __init__(self, service_name: str) -> None:
        super().__init__()
        self.service_name = service_name

    def format(self, record: logging.LogRecord) -> str:
        cid = CORRELATION_ID_VAR.get("")
        # RFC 3339 timestamp with milliseconds and UTC offset to match Rust's
        # tracing-subscriber JSON output.
        from datetime import datetime, timezone
        ts = datetime.fromtimestamp(record.created, tz=timezone.utc)
        timestamp = ts.strftime("%Y-%m-%dT%H:%M:%S.") + "{:03.0f}Z".format(
            record.msecs
        )
        entry: dict = {
            "timestamp": timestamp,
            "level": record.levelname,
            "target": record.name,
            "message": record.getMessage(),
            "service": self.service_name,
        }
        if cid:
            entry["correlation_id"] = cid
        if record.exc_info and record.exc_info[0] is not None:
            entry["exception"] = self.formatException(record.exc_info)
        # Preserve extra structured fields.
        _STANDARD = {
            "name", "msg", "args", "levelname", "levelno", "pathname",
            "filename", "module", "exc_info", "exc_text", "stack_info",
            "lineno", "funcName", "created", "msecs", "relativeCreated",
            "thread", "threadName", "processName", "process", "message",
            "taskName",
        }
        fields = {
            k: v for k, v in record.__dict__.items()
            if k not in _STANDARD and not k.startswith("_")
        }
        if fields:
            entry["fields"] = fields
        return json.dumps(entry, default=str)


class ArcanumTextFormatter(logging.Formatter):
    """Human-readable formatter that includes correlation_id when set."""

    def format(self, record: logging.LogRecord) -> str:
        cid = CORRELATION_ID_VAR.get("")
        base = super().format(record)
        if cid:
            return "[{}] {}".format(cid[:8], base)
        return base


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def _log_dir() -> Path:
    raw = os.environ.get("ARCANUM_LOG_DIR", "")
    if raw:
        return Path(raw)
    return Path.home() / ".arcanum" / "logs"


def _read_level() -> str:
    """Read log level from env vars with fallback chain."""
    for key in ("ARCANUM_LOG_LEVEL", "SEER_LOG_LEVEL", "TOME_LOG_LEVEL"):
        val = os.environ.get(key, "")
        if val:
            return val.upper()
    return "INFO"


# ---------------------------------------------------------------------------
# Public API
# ---------------------------------------------------------------------------

def configure_logging(service_name: str) -> None:
    """Configure the root logger for the given service.

    Reads ``ARCANUM_LOG_LEVEL``, ``ARCANUM_LOG_FORMAT``, ``ARCANUM_LOG_DIR``,
    ``ARCANUM_LOG_FILE``. Safe to call multiple times (idempotent after first
    call). Thread-safe.
    """
    global _CONFIGURED
    with _CONFIGURE_LOCK:
        if _CONFIGURED:
            return

        level_name = _read_level()
        level = getattr(logging, level_name, logging.INFO)

        log_format = os.environ.get("ARCANUM_LOG_FORMAT", "text").lower()
        file_enabled = os.environ.get("ARCANUM_LOG_FILE", "").lower() in (
            "1", "true", "yes",
        )

        handlers: list[logging.Handler] = []

        # Console handler — always to stderr (stdout is reserved for MCP stdio).
        console = logging.StreamHandler(sys.stderr)
        if log_format == "json":
            console.setFormatter(ArcanumJsonFormatter(service_name))
        else:
            console.setFormatter(
                ArcanumTextFormatter(
                    "%(asctime)s %(levelname)-8s %(name)s: %(message)s",
                )
            )
        handlers.append(console)

        # File handler with daily rotation.
        if file_enabled:
            log_path = _log_dir()
            log_path.mkdir(parents=True, exist_ok=True)
            fh = logging.handlers.TimedRotatingFileHandler(
                log_path / "{}.log".format(service_name),
                when="midnight",
                backupCount=7,
                encoding="utf-8",
            )
            # Always JSON in files for machine readability.
            fh.setFormatter(ArcanumJsonFormatter(service_name))
            handlers.append(fh)

        # OpenTelemetry OTLP log handler (lazy-imported, zero cost if not configured).
        otel_endpoint = os.environ.get("ARCANUM_OTEL_ENDPOINT", "")
        if otel_endpoint:
            otel_handler = _build_otel_handler(otel_endpoint, service_name)
            if otel_handler is not None:
                handlers.append(otel_handler)

        logging.basicConfig(level=level, handlers=handlers, force=True)
        _CONFIGURED = True


def get_logger(name: str) -> logging.Logger:
    """Return a logger, optionally prefixed with ``arcanum.``."""
    return logging.getLogger(name)


def set_correlation_id(cid: Optional[str] = None) -> str:
    """Set the correlation ID for the current async context.

    Generates a short UUID if *cid* is not provided. Returns the ID.
    """
    resolved = cid or uuid.uuid4().hex[:16]
    CORRELATION_ID_VAR.set(resolved)
    return resolved


def get_correlation_id() -> str:
    """Get the current correlation ID, generating one if absent."""
    cid = CORRELATION_ID_VAR.get("")
    if not cid:
        cid = set_correlation_id()
    return cid


# ---------------------------------------------------------------------------
# OpenTelemetry (lazy-imported)
# ---------------------------------------------------------------------------

def _build_otel_handler(
    endpoint: str, service_name: str
) -> Optional[logging.Handler]:
    """Build an OTLP log exporter handler.

    Returns ``None`` if the ``opentelemetry`` packages are not installed.
    """
    try:
        from opentelemetry.sdk._logs import LoggerProvider, LoggingHandler
        from opentelemetry.sdk._logs.export import BatchLogRecordProcessor
        from opentelemetry.exporter.otlp.proto.grpc._log_exporter import (
            OTLPLogExporter,
        )
        from opentelemetry.sdk.resources import Resource

        resource = Resource.create({"service.name": service_name})
        exporter = OTLPLogExporter(endpoint=endpoint, insecure=True)
        provider = LoggerProvider(resource=resource)
        provider.add_log_record_processor(BatchLogRecordProcessor(exporter))
        handler = LoggingHandler(
            level=logging.NOTSET, logger_provider=provider
        )
        return handler
    except ImportError:
        return None
