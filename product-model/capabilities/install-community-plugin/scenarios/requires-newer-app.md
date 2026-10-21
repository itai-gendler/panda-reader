---
kind: validation
routes:
  desktop: Desktop
steps:
  - text: The plugin's minimum app version is above this app's version
    kind: condition
    entities: []
    contexts:
      desktop:
        place: desktop::plugin-settings
  - text: Look at the plugin's Requires newer app button
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::plugin-settings
---

# The plugin needs a newer Panda Reader

## Trigger

The catalog entry needs a newer app version or another plugin API version.

## Outcome

Its button reads "Requires newer app" and does nothing.
