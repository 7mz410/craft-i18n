//! Test-only crate: see `tests/`. It asserts things about the translation files in `locales/`, so
//! a file that doesn't parse or a translation of a string that doesn't exist fails CI here, not in
//! an app.
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]

pub mod catalog;
pub mod locale;

/// The repository root (where `locales/` lives).
pub fn repo_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
