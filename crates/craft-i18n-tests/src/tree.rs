//! Checking every catalog under a checkout's `locales/`: `locales/<locale>/common.tsv` (shared
//! strings) and `locales/<locale>/<app>.tsv` (one app's strings and overrides).

use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use crate::catalog::{Row, read_catalog};
use crate::locale::{check_locale_name, plural_forms};

/// The shared catalog's file name.
pub const COMMON: &str = "common.tsv";

/// Every problem under `root/locales`, each prefixed by its path relative to `root`. No
/// `locales/` directory means no locales, and no problems.
pub fn check_tree(root: &Path) -> Vec<String> {
    let mut problems = Vec::new();
    let locales = match entries(&root.join("locales")) {
        Ok(locales) => locales,
        Err(e) => return vec![format!("locales: {e}")],
    };
    for locale in locales {
        let dir = root.join("locales").join(&locale);
        if !dir.is_dir() {
            problems.push(format!("locales/{locale}: not a locale directory"));
            continue;
        }
        if let Err(e) = check_locale_name(&locale) {
            problems.push(format!("locales/{locale}: {e}"));
            continue;
        }
        let forms = plural_forms(&locale).unwrap_or_else(|e| {
            problems.push(format!("locales/{locale}: {e}"));
            1
        });
        let files = match entries(&dir) {
            Ok(files) => files,
            Err(e) => {
                problems.push(format!("locales/{locale}: {e}"));
                continue;
            }
        };
        let shared: HashSet<Row> = read(&dir.join(COMMON), forms)
            .map(|(rows, _)| rows.into_iter().map(key).collect())
            .unwrap_or_default();
        for file in files {
            let path = format!("locales/{locale}/{file}");
            if !is_catalog_name(&file) {
                problems.push(format!(
                    "{path}: not a catalog; name it {COMMON} or <app>.tsv (lowercase, digits, -)"
                ));
                continue;
            }
            let (rows, text_problems) = match read(&dir.join(&file), forms) {
                Ok(read) => read,
                Err(e) => {
                    problems.push(format!("{path}: {e}"));
                    continue;
                }
            };
            problems.extend(text_problems.into_iter().map(|p| format!("{path}: {p}")));
            if file != COMMON {
                for row in rows
                    .into_iter()
                    .filter(|r| shared.contains(&key(r.clone())))
                {
                    problems.push(format!(
                        "{path}: line {}: same translation as {COMMON}; remove the override",
                        row.line
                    ));
                }
            }
        }
    }
    problems
}

/// One line per catalog, `<locale>/<file>: <rows>/<max>`, where `<max>` is the most rows any
/// locale's catalog of that name has. A report only: unreadable files count as empty.
pub fn coverage(root: &Path) -> Vec<String> {
    let mut counts = Vec::new();
    for locale in entries(&root.join("locales")).unwrap_or_default() {
        let dir = root.join("locales").join(&locale);
        let forms = plural_forms(&locale).unwrap_or(1);
        for file in entries(&dir).unwrap_or_default() {
            if is_catalog_name(&file) {
                let rows = read(&dir.join(&file), forms).map_or(0, |(rows, _)| rows.len());
                counts.push((locale.clone(), file, rows));
            }
        }
    }
    let mut max: BTreeMap<&str, usize> = BTreeMap::new();
    for (_, file, rows) in &counts {
        let m = max.entry(file).or_default();
        *m = (*m).max(*rows);
    }
    counts
        .iter()
        .map(|(locale, file, rows)| {
            let m = max.get(file.as_str()).copied().unwrap_or(0);
            format!("{locale}/{file}: {rows}/{m}")
        })
        .collect()
}

/// A catalog's rows and row problems, plus a problem if the clean-room rule is missing.
fn read(path: &Path, forms: usize) -> Result<(Vec<Row>, Vec<String>), String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let (rows, mut problems) = read_catalog(&text, forms);
    if !text.lines().any(|l| l.starts_with("# Clean room:")) {
        problems.insert(
            0,
            "no \"# Clean room:\" line; copy the header from docs/catalog-format.md".into(),
        );
    }
    Ok((rows, problems))
}

/// A row without its line number, to compare rows across files.
fn key(row: Row) -> Row {
    Row { line: 0, ..row }
}

/// `common.tsv`, or `<app>.tsv` with `<app>` in `[a-z0-9]+(-[a-z0-9]+)*`.
fn is_catalog_name(file: &str) -> bool {
    file.strip_suffix(".tsv").is_some_and(|app| {
        app.split('-').all(|part| {
            !part.is_empty()
                && part
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        })
    })
}

/// The names in `dir`, sorted, without dotfiles. A missing `dir` has none.
fn entries(dir: &Path) -> std::io::Result<Vec<String>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut names = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let name = entry?.file_name().to_string_lossy().into_owned();
        if !name.starts_with('.') {
            names.push(name);
        }
    }
    names.sort();
    Ok(names)
}
