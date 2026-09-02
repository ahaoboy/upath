//! Core uniqueness algorithm behind [`crate::upath`].

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use crate::ext::split;
use crate::style::{NumberStyle, detect_version, render};

/// Resolve `path` to the nearest free (non-colliding) sibling name.
///
/// If `path` does not exist it is returned unchanged.
pub(crate) fn resolve(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        return Ok(path.to_path_buf());
    }

    let Some(file_name) = path.file_name() else {
        // e.g. a filesystem root like `/` or `C:\` has nothing to number.
        return Err(Error::MissingFileName);
    };
    let parent = path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));

    match file_name.to_str() {
        Some(name) => resolve_utf8(&parent, name),
        None => resolve_non_utf8(&parent, file_name),
    }
}

/// Fast path for the common, valid-UTF-8 file-name case.
fn resolve_utf8(parent: &Path, name: &str) -> Result<PathBuf> {
    let (base, ext) = split(name);

    // If the existing name already carries a number, keep its bracket style
    // (and zero-padding) and continue from just above it; otherwise start a
    // fresh round-bracket suffix at 1.
    let (core, style, pad, mut n) = match detect_version(base) {
        Some((core, version)) => (core, version.style, version.pad, version.number),
        None => (base.to_string(), NumberStyle::DEFAULT, None, 1),
    };

    loop {
        let file_name = format!("{core}{}{ext}", render(style, n, pad));
        let candidate = parent.join(&file_name);
        if !candidate.exists() {
            return Ok(candidate);
        }
        n = n.checked_add(1).ok_or(Error::Overflow)?;
    }
}

/// Fallback for file names that are not valid UTF-8. Style/extension detection
/// is impossible here, so a plain suffix is appended to the whole name.
fn resolve_non_utf8(parent: &Path, file_name: &OsStr) -> Result<PathBuf> {
    let mut n = 1u64;
    loop {
        let mut base = file_name.to_os_string();
        base.push(render(NumberStyle::DEFAULT, n, None));
        let candidate = parent.join(base);
        if !candidate.exists() {
            return Ok(candidate);
        }
        n = n.checked_add(1).ok_or(Error::Overflow)?;
    }
}
