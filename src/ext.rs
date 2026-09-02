//! Extension splitting.
//!
//! The number is always inserted at the **first `.`**, which keeps the whole
//! dot-suffix together: `a.tar.xz` → `a` + `.tar.xz`, so the duplicate is
//! `a(1).tar.xz` rather than `a.tar(1).xz`. A leading dot that opens a plain
//! dot-file (`.gitignore`, `.a.b.c`) is part of the name, **not** a separator.

/// Split `name` into `(base, ext)`. `ext` keeps its leading dot and is empty
/// when there is no dot to split on.
///
/// * `a.b.c`        → (`a`, `.b.c`)
/// * `.a.b.c`       → (`.a`, `.b.c`)   — a leading dot is skipped
/// * `.gitignore`   → (`.gitignore`, ``)
/// * `Makefile`     → (`Makefile`, ``)
pub(crate) fn split(name: &str) -> (&str, &str) {
    // A leading dot belongs to the file name; start searching after it.
    let start = usize::from(name.starts_with('.'));
    if let Some(offset) = name[start..].find('.') {
        let idx = start + offset;
        return (&name[..idx], &name[idx..]);
    }
    (name, "")
}
