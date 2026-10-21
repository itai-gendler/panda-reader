---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: The catalog version is newer than the installed one
    kind: condition
    entities:
      - entity: article-plugin
        effect: reads
        facts:
          - Version
    contexts:
      desktop:
        place: desktop::plugin-settings
  - text: Choose Update on the plugin
    kind: actor
    actor: reader
    entities:
      - entity: article-plugin
        effect: reads
        facts:
          - Name
    contexts:
      desktop:
        place: desktop::plugin-settings
  - text: Download and verify the new version and replace the installed one
    kind: product
    actor: reader
    entities:
      - entity: article-plugin
        effect: changes
        facts:
          - Version
          - Domains
          - Path prefixes
          - Priority
          - Host capabilities
          - Network hosts
    contexts:
      desktop:
        place: desktop::plugin-settings
---

# Update an installed community plugin

## Trigger

The catalog lists a newer version of an installed plugin, so its row shows Update.

## Outcome

The verified newer version replaces the installed one; updates are only installed when the Reader chooses Update.
