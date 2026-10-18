# Using craft-i18n in a Crafting App

craft-i18n is a **fully optional** source of translations. An app must build, test and run without
it, in English; with it, the app embeds the catalogs in `locales/`. The code that reads them is the
`craft-i18n` crate in craft-libs, which apps already depend on
([ADR 0002](adr/0002-runtime-in-craft-libs-data-in-craft-i18n.md)).

## Why not a Cargo dependency

Cargo resolves every dependency in `Cargo.toml`, optional ones included. A git dependency on this
repository would make every build (offline, in a fork, on a mirror) need it. So apps never list it
in `Cargo.toml`; the build option is an environment variable instead, as for craft-fonts.

## The build option: `CRAFT_I18N_DIR`

```sh
git clone https://github.com/benandez/craft-i18n ../craft-i18n
CRAFT_I18N_DIR=$PWD/../craft-i18n cargo build --release
```

The app's `build.rs` writes one static into `OUT_DIR`, always, so the app needs no `cfg`:

```rust
/// (locale, shared catalog, this app's catalog), one entry per locale; "" for a missing file.
pub static CRAFT_I18N: &[(&str, &str, &str)] = &[
    ("fr", include_str!("/abs/craft-i18n/locales/fr/common.tsv"), include_str!("/abs/craft-i18n/locales/fr/wordcraft.tsv")),
    // …
];
```

- **Unset** (the default): the static is empty. The app is English-only.
- **Set to a checkout** (it has a `locales/` directory): one entry per locale that has this app's
  own catalog (`locales/<locale>/<app>.tsv`), embedded with `include_str!` on absolute paths. A
  locale with only shared strings is left out, so an app never offers a language it has barely
  been translated into.
- **Set but not a checkout:** `cargo::warning`, and the static is empty.
- **`CRAFT_I18N_REQUIRED=1`** with a missing or invalid checkout: `cargo::error`. Releases set it,
  so a release never ships English-only by accident.
- `cargo::rerun-if-env-changed` for both variables, and `cargo::rerun-if-changed` for every
  embedded file and the `locales/` directory.
- Web (wasm) builds embed the same locales as desktop.

Use an **absolute** path: Cargo runs build scripts in the crate's own directory.

## What the app shows

The app passes `CRAFT_I18N` to the craft-libs crate, with the codes of its own language list: only
those are offered (an embedded locale the app isn't ready for, such as a right-to-left one, stays a
fallback only). For `pt-br` it looks up: `pt-br` app catalog → `pt-br` shared → `pt` app → `pt`
shared → the English source text. An app catalog's row wins over
the shared one in its own locale only. A row the crate doesn't understand (a newer `@` context) is
skipped and logged, and that string shows in English.

## Stale and missing strings

craft-i18n can't see app code. With `CRAFT_I18N_DIR` set, each app's tests report the English
source texts that have no translation, and the translations of source texts the app no longer
uses. The report never fails the build.

## Release builds

Whether craft-common-actions' release workflow checks out craft-i18n and sets
`CRAFT_I18N_DIR` and `CRAFT_I18N_REQUIRED=1`, as it does for craft-fonts, is a pending maintainer
decision.
