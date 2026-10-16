//! The Crafting Apps' shared translations.
//!
//! The strings live in this repository's `locales/` directory as [Fluent](https://projectfluent.org)
//! files, one directory per locale:
//!
//! - `locales/<locale>/common.ftl`: strings shared by every app.
//! - `locales/<locale>/<app>.ftl`: strings for one app, and overrides of shared ones.
//!
//! [`SOURCE_LOCALE`] is the reference every other locale translates. This crate finds a craft-i18n
//! checkout ([`dir_from_env`]), lists its locales ([`locales`]) and gives the files an app loads,
//! in order ([`files`]).
//!
//! The apps do **not** depend on this crate, for the same reason they don't depend on
//! craft-fonts: Cargo resolves even optional git dependencies. See `docs/integration.md`.
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]

use std::path::{Path, PathBuf};

/// The locale every other locale translates from.
pub const SOURCE_LOCALE: &str = "en";

/// The file of strings shared by every app.
pub const COMMON: &str = "common.ftl";

/// The environment variable (and build option) naming a craft-i18n checkout.
pub const DIR_ENV: &str = "CRAFT_I18N_DIR";

/// The craft-i18n checkout named by `CRAFT_I18N_DIR`, if it is set and holds the source locale.
pub fn dir_from_env() -> Option<PathBuf> {
    let dir = PathBuf::from(std::env::var_os(DIR_ENV)?);
    dir.join("locales")
        .join(SOURCE_LOCALE)
        .join(COMMON)
        .is_file()
        .then_some(dir)
}

/// The locale directories of the checkout at `dir`, sorted.
pub fn locales(dir: &Path) -> std::io::Result<Vec<String>> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir.join("locales"))? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            out.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    out.sort();
    Ok(out)
}

/// The files `app` loads for `locale`, in load order: shared strings, then the app's own, which
/// override shared ones with the same id. Files that don't exist are left out.
pub fn files(dir: &Path, locale: &str, app: &str) -> Vec<PathBuf> {
    let base = dir.join("locales").join(locale);
    [base.join(COMMON), base.join(format!("{app}.ftl"))]
        .into_iter()
        .filter(|p| p.is_file())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    #[test]
    fn the_source_locale_exists() {
        assert!(
            locales(&root())
                .unwrap()
                .contains(&SOURCE_LOCALE.to_owned())
        );
    }

    #[test]
    fn shared_strings_load_before_app_strings() {
        let f = files(&root(), SOURCE_LOCALE, "no-such-app");
        assert_eq!(f, [root().join("locales/en/common.ftl")]);
    }

    #[test]
    fn a_missing_checkout_is_an_error() {
        assert!(locales(Path::new("/nonexistent/craft-i18n")).is_err());
        assert!(files(Path::new("/nonexistent/craft-i18n"), "en", "wordcraft").is_empty());
    }
}
