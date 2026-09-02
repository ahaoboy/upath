//! `upath` — generate an **available** (non-colliding) file path.
//!
//! Given a candidate path that already exists on disk, `upath` returns the
//! nearest sibling name that is free by inserting or incrementing a numeric
//! suffix:
//!
//! ```text
//! a.txt           exists -> a(1).txt
//! a(1).txt        exists -> a(2).txt
//! a[3].txt        exists -> a[4].txt        (bracket style is preserved)
//! 报告（1）.txt   exists -> 报告（2）.txt    (Chinese brackets work too)
//! a.tar.xz        exists -> a(1).tar.xz      (split at the first dot)
//! a.b.c           exists -> a(1).b.c
//! .a.b.c          exists -> .a(1).b.c        (a leading dot is part of the name)
//! .gitignore      exists -> .gitignore(1)
//! ```
//!
//! If the given path does **not** exist it is returned unchanged.
//!
//! # Usage
//!
//! ```no_run
//! use upath::upath;
//!
//! let next = upath("/tmp/a.txt")?;
//! # Ok::<(), upath::Error>(())
//! ```
//!
//! The input accepts anything implementing [`AsRef<Path>`](std::path::Path):
//! `&str`, `String`, `&Path` or `PathBuf`.
//!
//! # Rules
//!
//! * The suffix is inserted at the **first `.`**, so only the first dot-part is
//!   treated as the name and the rest is kept as a whole tail: `a.tar.xz` →
//!   `a(1).tar.xz`, `report.2024.txt` → `report(1).2024.txt`.
//! * A leading dot that opens a dot-file is part of the name, not a separator:
//!   `.gitignore` → `.gitignore(1)`, `.a.b.c` → `.a(1).b.c`.
//! * An existing numbered sibling keeps its bracket style *and* zero-padding:
//!   `a[042].txt` → `a[043].txt`.
//! * Supported brackets are round `()`, square `[]`, curly `{}`, full-width
//!   `（）`, full-width square `［］`, Chinese `【】` and corner `「」`.
//!
//! # Design notes
//!
//! * Detection is **stateless** — nothing is configured globally and no
//!   per-directory state is kept between calls.
//! * Duplicate detection probes each candidate with an `exists` check, which
//!   keeps the crate free of `unsafe`, side effects and extra dependencies.

#![forbid(unsafe_code)]

mod error;
mod ext;
mod style;
mod unique;

pub use crate::error::{Error, Result};

use std::path::{Path, PathBuf};

/// Resolve `path` to the nearest free sibling path.
///
/// Returns the input unchanged when it does not already exist.
///
/// # Errors
///
/// Returns [`Error::MissingFileName`] if the path has no file-name component,
/// [`Error::Io`] on I/O failures and [`Error::Overflow`] if the numeric suffix
/// counter would exceed `u64`.
pub fn upath<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    unique::resolve(path.as_ref())
}
