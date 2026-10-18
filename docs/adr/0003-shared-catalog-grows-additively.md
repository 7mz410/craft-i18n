# The shared catalog grows additively

From milestone 3b on, migrating an app never changes an existing `common.tsv` row. Where the new
app translates a shared string differently, its wording becomes an override in its own catalog.
New shared strings are promoted only from strings the existing shared catalog doesn't hold yet:
used by two or more apps and translated alike in most locales they share, the translation most
apps use winning. Rows already shared were reviewed in 3a, and re-seeding would churn every
app's overrides each time an app moves.

This narrows the spec's "most-used translation wins" (user story 41): for a string shared before
an app migrated, the earlier apps' wording stays even when later apps agree on another.

## Considered options

- **Re-seed `common.tsv` from every migrated app at each migration**: rejected, winners can flip,
  so WordCraft's and PdfCraft's overrides churn and the reviewed shared strings are reviewed again.
