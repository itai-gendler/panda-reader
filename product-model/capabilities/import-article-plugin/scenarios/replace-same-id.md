---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Browse ZIP, pick the archive and choose Import
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::plugin-settings
  - text: A plugin with the same ID is installed
    kind: condition
    entities:
      - entity: article-plugin
        effect: reads
        facts:
          - ID
    contexts:
      desktop:
        place: desktop::plugin-settings
  - text: Replace the installed copy with the new one
    kind: product
    actor: reader
    entities:
      - entity: article-plugin
        effect: changes
        facts:
          - Name
          - Version
          - Format
          - Stage
          - Domains
          - Path prefixes
          - Priority
          - Host capabilities
          - Network hosts
    contexts:
      desktop:
        place: desktop::plugin-settings
---

# Import a new version of an installed plugin

## Trigger

The imported plugin has the ID of a plugin already installed.

## Outcome

The new copy replaces the installed one; the old one is restored if the replacement cannot be completed.
