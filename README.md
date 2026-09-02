# upath

Generate an **available** (non-colliding) file path by appending or incrementing
a numeric suffix. Written in Rust, no `unsafe`, no dependencies.

Given a candidate that **already exists**, returns the nearest free sibling:

```
a.txt       exists  -> a(1).txt
a(1).txt    exists  -> a(2).txt
a[3].txt    exists  -> a[4].txt       # bracket style is preserved
a.tar.xz    exists  -> a(1).tar.xz    # split at the first dot
.gitignore  exists  -> .gitignore(1)
```

If the path does **not** exist it is returned unchanged.

## Library

```rust
use upath::upath;

let next = upath("/tmp/a.txt")?;   // a(1).txt if a.txt exists
```

The input accepts anything implementing `AsRef<Path>`: `&str`, `String`,
`&Path`, `PathBuf`.

## CLI

Takes a single path and prints the available path:

```
$ touch a.txt
$ upath a.txt
a(1).txt
```

