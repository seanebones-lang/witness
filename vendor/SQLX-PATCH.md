# SQLx SQLite compatibility patch

Upstream: sqlx-sqlite 0.8.6 from https://crates.io/api/v1/crates/sqlx-sqlite/0.8.6/download

Original crate SHA-256: `c2d12fe70b2c1b4401038055f90f151b78208de1f9f89a7dbfd41587a10c3eea`. All crate source and license bytes are preserved. The sole upstream-file modification is normalized Cargo.toml: libsqlite3-sys `0.30.1` → `=0.39.1`. Cargo.toml.orig is retained as original upstream provenance, not the active build manifest.

This avoids RUSTSEC-2026-0459 (crafted database/WAL memory exposure) while preserving Witness’s SQLx 0.8 query/migration APIs. Stable SQLx 0.9.0 still caps the binding below 0.38, so upgrading SQLx alone does not fix it. This patched binding bundles SQLite 3.54.1; no system-library fallback or advisory suppression is used.

The binding requires Rust 1.88 or newer; build with current stable Rust as before. Review and remove this patch when a compatible upstream release supports the patched binding. Preserve upstream MIT/Apache licenses included in this directory.
