---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Check for updates under Community plugins
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::plugin-settings
  - text: Download the community catalog and compare it with the installed plugins
    kind: product
    actor: reader
    entities:
      - entity: article-plugin
        effect: reads
        facts:
          - ID
          - Version
    contexts:
      desktop:
        place: desktop::plugin-settings
---

# Check the community catalog

## Trigger

The Reader opens Plugins or chooses Check for updates.

## Outcome

Each community plugin shows Install, Installed, Update or Requires newer app, with the HTTPS hosts it may contact.
