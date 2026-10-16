//! The checker run over this repository's `locales/`, and over small fixture trees.

use std::path::{Path, PathBuf};

use craft_i18n_tests::tree::{check_tree, coverage};

const H: &str = "# Clean room: written from the plain meaning.\n";

#[test]
fn repository_catalogs_pass() {
    assert_eq!(
        check_tree(&craft_i18n_tests::repo_root()),
        Vec::<String>::new()
    );
}

/// Prints the coverage report (`cargo test coverage_report -- --nocapture`); never fails.
#[test]
fn coverage_report() {
    for line in coverage(&craft_i18n_tests::repo_root()) {
        println!("{line}");
    }
}

/// A fresh tree under the test target directory holding `files` (path relative to the root).
fn fixture(name: &str, files: &[(&str, &str)]) -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    for (path, text) in files {
        let path = root.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    root
}

fn assert_problems(name: &str, files: &[(&str, &str)], expected: &[&str]) {
    let problems = check_tree(&fixture(name, files));
    assert_eq!(problems.len(), expected.len(), "{problems:?}");
    for (problem, prefix) in problems.iter().zip(expected) {
        assert!(problem.starts_with(prefix), "{problem:?} vs {prefix:?}");
    }
}

#[test]
fn clean_tree_passes() {
    let common = format!("{H}\tSave\tEnregistrer\n");
    let app = format!("{H}\tSave\tSauvegarder\n");
    assert_problems(
        "clean",
        &[
            ("locales/fr/common.tsv", &common),
            ("locales/fr/wordcraft.tsv", &app),
            ("locales/fr/.DS_Store", ""),
        ],
        &[],
    );
}

#[test]
fn bad_row_names_file_and_line() {
    let common = format!("{H}\tSave\n");
    assert_problems(
        "bad-row",
        &[("locales/fr/common.tsv", &common)],
        &["locales/fr/common.tsv: line 2:"],
    );
}

#[test]
fn bad_locale_name() {
    assert_problems(
        "bad-locale",
        &[("locales/pt_BR/common.tsv", H)],
        &["locales/pt_BR:"],
    );
}

#[test]
fn stray_file() {
    assert_problems(
        "stray",
        &[("locales/fr/common.ftl", "")],
        &["locales/fr/common.ftl:"],
    );
}

#[test]
fn bad_app_file_name() {
    assert_problems(
        "bad-app",
        &[("locales/fr/Word_Craft.tsv", H)],
        &["locales/fr/Word_Craft.tsv:"],
    );
}

#[test]
fn missing_clean_room_rule() {
    assert_problems(
        "no-clean-room",
        &[("locales/fr/common.tsv", "\tSave\tEnregistrer\n")],
        &["locales/fr/common.tsv:"],
    );
}

#[test]
fn redundant_override() {
    let same = format!("{H}\tSave\tEnregistrer\n");
    assert_problems(
        "redundant",
        &[
            ("locales/fr/common.tsv", &same),
            ("locales/fr/wordcraft.tsv", &same),
        ],
        &["locales/fr/wordcraft.tsv: line 2:"],
    );
}

#[test]
fn plural_count_by_locale() {
    let common = format!("{H}@plural\t{{n}} a|{{n}} b\t{{n}} x|{{n}} y\n");
    assert_problems(
        "plural-ja",
        &[("locales/ja/common.tsv", &common)],
        &["locales/ja/common.tsv: line 2:"],
    );
}

#[test]
fn no_locales_dir() {
    assert_problems("empty", &[], &[]);
}

#[test]
fn coverage_counts_rows_against_largest_locale() {
    let fr = format!("{H}\tSave\tEnregistrer\n\tUndo\tAnnuler\n");
    let de = format!("{H}\tSave\tSpeichern\n");
    let report = coverage(&fixture(
        "coverage",
        &[
            ("locales/fr/common.tsv", &fr),
            ("locales/de/common.tsv", &de),
        ],
    ));
    assert!(
        report.contains(&"de/common.tsv: 1/2".to_owned()),
        "{report:?}"
    );
    assert!(
        report.contains(&"fr/common.tsv: 2/2".to_owned()),
        "{report:?}"
    );
}
