craft-i18n
==========

Shared translations for the Crafting Apps.

The apps share a lot of strings. Keeping them here means each one is translated once, and every
app picks it up. An app can still override a shared string, or add its own, in its own file.

## What's here

| Path | What |
|---|---|
| `locales/<locale>/common.ftl` | Strings shared by every app, in [Fluent](https://projectfluent.org) syntax. |
| `locales/<locale>/<app>.ftl` | Strings for one app, and its overrides of shared ones. |
| `locales/en/` | The source locale: every other locale translates these files. |
| `ATTRIBUTION.md` | Translators and licences. |
| `crates/craft-i18n` | Rust: finding a checkout (`CRAFT_I18N_DIR`), listing locales, and the files an app loads, in order. |
| `crates/craft-i18n-tests` | Tests asserted on the files: they parse, ids are unique, translations only use ids the source defines. |
| `docs/integration.md` | How an app uses craft-i18n as a fully optional build input. |

## How the apps use it

**Fully optional, never required.** Same rule as craft-fonts: the apps don't list craft-i18n in
`Cargo.toml`. Building with `CRAFT_I18N_DIR=<path to this checkout>` makes an app use these
strings; without it, the app builds and runs as before. See
[`docs/integration.md`](docs/integration.md).

## Translate

Fluent files are plain text, one `id = text` per line. To add or fix a translation, edit
`locales/<locale>/<file>.ftl` (GitHub's web editor is fine) and open a pull request. Copy the
ids from the matching file in `locales/en/`; CI rejects ids `en` doesn't have.

## Develop

```sh
cargo test --workspace   # all translation assertions
```

Code and translations are MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`).
