# craft-i18n roadmap

> **Last updated:** 2026-10-17 · **Change:** minor (milestones 1 and 2 done; WordCraft and PdfCraft
> catalogs migrated)

Shared translations for the storytold Crafting Apps: one catalog per locale for shared strings,
one per app, checked in CI and embedded by each app at build time. Vocabulary is in
[`GLOSSARY.md`](GLOSSARY.md); decisions are in [`docs/adr/`](docs/adr/).

## Milestones

| # | Milestone | Where | Status |
|---|---|---|---|
| 1 | TSV catalog format and strict checker; Fluent and the unusable library crate removed; translator and integration docs | this repo | done (PR #1) |
| 2 | `craft-i18n` runtime crate: parser, lookup, fallback, plurals (`intl_pluralrules`), locale matching, pseudo locale | craft-libs | done on a fork; upstream PR pending |
| 3a | Migrate WordCraft and PdfCraft (proves the runtime on the largest catalogs; seeds the shared catalog) | apps | catalogs moved here; app wiring waits on craft-libs |
| 3b | Migrate the other TSV apps: PhotoCraft, VectorCraft, FilmCraft, CADCraft, craft-launcher | apps | not started |
| 3c | Convert and migrate FigCraft (TOML), LightCraft (JSON), DesignCraft, GridCraft, EffectCraft (Rust tables) | apps | not started |
| 3d | Add translation to DeckCraft and SoundCraft | apps | not started |

Each app migration: confirm the app is MIT OR Apache-2.0, move its catalogs here (lowercase BCP 47
names, origin recorded in `ATTRIBUTION.md`), add the `build.rs` recipe from
[`docs/integration.md`](docs/integration.md), switch to the runtime crate, delete the app's copied
i18n module, and add the app-side stale/missing string report.

Known migration fixes: PdfCraft's Turkish `@plural` rows listed one form, CLDR (and CI) need two;
done in 3a by repeating the form (Turkish nouns stay singular after numerals). The runtime crate
must pin the same `intl_pluralrules` version as the checker, or their plural form counts can disagree.

## Apps

As surveyed on 2026-10-16. Locale counts exclude English.

| App | Format today | Locales | Plurals | Migrated |
|---|---|---|---|---|
| WordCraft | TSV | 22 | none | catalogs |
| PdfCraft | TSV | 22 | `@plural` | catalogs |
| PhotoCraft | TSV | 20 | `@plural` | no |
| VectorCraft | TSV | 19 | `@plural` | no |
| FilmCraft | TSV | 11 | none | no |
| CADCraft | TSV | 12 | `@plural` + own rules | no |
| craft-launcher | TSV | 10 | `@plural` | no |
| FigCraft | TOML | 14 | CLDR | no |
| LightCraft | JSON | 13 | minimal | no |
| DesignCraft | Rust tables | 11 | own rules | no |
| GridCraft | Rust `match` | 9 | none | no |
| EffectCraft | Rust tables | 10 | none | no |
| DeckCraft | none | 0 | – | no |
| SoundCraft | none | 0 | – | no |

## Open decisions

- **Release builds** (maintainer): whether craft-common-actions' release workflow checks out
  craft-i18n at a pinned ref and sets `CRAFT_I18N_DIR` and `CRAFT_I18N_REQUIRED=1`, as it does for
  craft-fonts. Until then, releases are English-only unless an app sets them itself.
- **Repository home**: moving from a personal account to storytold.
- **Progress-docs shape**: align this page with the craftrules progress-docs standard once it is
  available to this repository.

## Change log

| Date | Change | Summary |
|---|---|---|
| 2026-10-16 | major | First roadmap: milestones 1–3d, per-app survey, open decisions |
| 2026-10-17 | minor | Milestones 1 and 2 done; WordCraft and PdfCraft catalogs migrated (227 shared strings, 28 locales) |
