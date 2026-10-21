---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: Choose Install on the plugin
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::plugin-settings
  - text: Reject the download because it does not match the catalog
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::plugin-settings
---

# The download fails verification

## Trigger

A downloaded file does not match the catalog's SHA-256 hash or its manifest differs from the catalog entry.

## Outcome

The error is shown and the installed plugins are unchanged.
