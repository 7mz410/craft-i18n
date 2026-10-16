//! Assertions on the translation files themselves.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use craft_i18n::{SOURCE_LOCALE, locales};
use fluent_syntax::ast::Entry;

/// Every `.ftl` file of a locale.
fn ftl_files(locale: &str) -> Vec<PathBuf> {
    let dir = craft_i18n_tests::repo_root().join("locales").join(locale);
    let mut out: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "ftl"))
        .collect();
    out.sort();
    out
}

/// The message and term ids a file defines, failing on parse errors and duplicates.
fn ids(path: &Path) -> BTreeSet<String> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let resource = fluent_syntax::parser::parse(text.as_str())
        .unwrap_or_else(|(_, errors)| panic!("{}: {errors:?}", path.display()));
    let mut ids = BTreeSet::new();
    for entry in resource.body {
        let id = match entry {
            Entry::Message(m) => m.id.name.to_owned(),
            Entry::Term(t) => format!("-{}", t.id.name),
            _ => continue,
        };
        assert!(
            ids.insert(id.clone()),
            "{}: `{id}` defined twice",
            path.display()
        );
    }
    ids
}

#[test]
fn every_file_parses_without_duplicate_ids() {
    for locale in locales(&craft_i18n_tests::repo_root()).unwrap() {
        for f in ftl_files(&locale) {
            ids(&f);
        }
    }
}

#[test]
fn translations_only_use_ids_the_source_locale_defines() {
    let root = craft_i18n_tests::repo_root();
    for locale in locales(&root).unwrap() {
        if locale == SOURCE_LOCALE {
            continue;
        }
        for f in ftl_files(&locale) {
            let source = root
                .join("locales")
                .join(SOURCE_LOCALE)
                .join(f.file_name().unwrap());
            assert!(
                source.is_file(),
                "{}: no {SOURCE_LOCALE} file to translate",
                f.display()
            );
            let extra: Vec<_> = ids(&f).difference(&ids(&source)).cloned().collect();
            assert!(
                extra.is_empty(),
                "{}: ids not in {SOURCE_LOCALE}: {extra:?}",
                f.display()
            );
        }
    }
}
