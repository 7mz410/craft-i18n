# Using craft-i18n in a Crafting App

craft-i18n is a **fully optional** source of strings. An app must build, test and run without it;
with it, the app uses the translations in `locales/`.

## Why not a Cargo dependency

Cargo resolves every dependency in `Cargo.toml`, optional ones included, even when the feature
that enables them is off. A git dependency on craft-i18n would make every build (offline, in a
fork, on a mirror) need this repository. So apps never list craft-i18n in `Cargo.toml`. The build
option is an environment variable instead.

## The build option: `CRAFT_I18N_DIR`

```sh
git clone https://github.com/benandez/craft-i18n ../craft-i18n
CRAFT_I18N_DIR=$PWD/../craft-i18n cargo build --release
```

- **Unset** (the default): nothing changes. The app uses its own built-in strings.
- **Set to a craft-i18n checkout:** the app loads, for each locale, `locales/<locale>/common.ftl`
  then `locales/<locale>/<app>.ftl`; an id in the app file overrides the shared one.
- **Set but not a checkout:** the build prints a `cargo::warning` and carries on without
  craft-i18n, unless `CRAFT_I18N_REQUIRED=1` is also set, which makes it a build error.

Use an **absolute** path: Cargo runs build scripts in the crate's own directory.

## Open

The `build.rs` recipe (embed the files vs. read at run time), the Fluent runtime the apps use, and
how apps fall back from a missing translation are still to be decided. Record them here once the
spec settles them.
