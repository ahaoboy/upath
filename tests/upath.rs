//! Input/output scenario tests for the public `upath` API.
//!
//! Uses a self-cleaning scratch directory under the system temp folder so no
//! external dependencies are needed.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use upath::upath;

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// A unique scratch directory that cleans itself up on drop.
struct Scratch {
    root: PathBuf,
}

impl Scratch {
    fn new() -> Scratch {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("upath-tests-{}-{id}", std::process::id()));
        // Best effort: clear leftovers from a previously crashed run.
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("create scratch dir");
        Scratch { root }
    }

    /// Run `name` through `upath` and return the resulting file name, asserting
    /// the result collides with nothing on disk.
    fn resolve(&self, name: &str) -> String {
        let out = upath(self.root.join(name)).expect("upath");
        assert!(!out.exists(), "result must not exist: {out:?}");
        out.file_name().unwrap().to_str().unwrap().to_string()
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn input_output_scenarios() {
    let scratch = Scratch::new();

    // A path that does not exist is returned unchanged.
    assert_eq!(scratch.resolve("free.txt"), "free.txt");

    // (input, additional siblings occupying the directory, expected output).
    // `input` itself always exists — that is what triggers the search — so it
    // is written by the test; only genuinely extra context is listed here.
    let cases: &[(&str, &[&str], &str)] = &[
        // A mixed `(1)[2]` suffix: only the trailing group is the version.
        ("a (1)[2].txt", &[], "a (1)[3].txt"),
        // Plain first collision.
        ("a.txt", &[], "a(1).txt"),
        // An existing round-bracket suffix is continued.
        ("b.txt", &["b(1).txt", "b(2).txt"], "b(3).txt"),
        // Square brackets are detected and continued.
        ("c[1].txt", &[], "c[2].txt"),
        // Split at the FIRST dot: `.tar.xz` is kept whole.
        ("d.tar.xz", &[], "d(1).tar.xz"),
        // Multi-dot fallback.
        ("e.b.c", &[], "e(1).b.c"),
        // A leading dot belongs to the name.
        (".a.b.c", &[], ".a(1).b.c"),
        // Dot-files get the suffix at the very end.
        (".gitignore", &[], ".gitignore(1)"),
        // No extension at all.
        ("Makefile", &[], "Makefile(1)"),
        // Zero padding is preserved.
        ("f(007).txt", &[], "f(008).txt"),
        // Chinese full-width brackets.
        ("报告（1）.txt", &[], "报告（2）.txt"),
    ];

    for (input, siblings, expected) in cases {
        fs::write(scratch.root.join(input), b"").expect("touch input");
        for sibling in *siblings {
            fs::write(scratch.root.join(sibling), b"").expect("touch sibling");
        }
        assert_eq!(scratch.resolve(input), *expected, "input: {input}");
    }
}
