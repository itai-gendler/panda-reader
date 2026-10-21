---
kind: validation
routes:
  desktop: Desktop
steps:
  - text: Pick a plugin folder and choose Import
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::plugin-settings
  - text: Validate the plugin and refuse it with the reason
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::plugin-settings
---

# Refuse an invalid plugin

## Trigger

The plugin uses a builtin. ID, an unsupported API version, a payload of the wrong kind, an undeclared capability, or the archive breaks the size and entry limits.

## Outcome

The reason is shown (for example "built-in plugin IDs cannot be imported") and nothing is installed.
