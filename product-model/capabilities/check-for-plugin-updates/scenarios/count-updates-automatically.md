---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: An automatic update check is due
    kind: condition
    unattended: true
    entities:
      - entity: settings
        effect: reads
        facts:
          - Check for updates automatically
    contexts:
      desktop:
        place: desktop::about
  - text: Download the community catalog and count installed plugins with a newer version
    kind: product
    entities:
      - entity: article-plugin
        effect: reads
        facts:
          - ID
          - Version
    contexts:
      desktop:
        place: desktop::about
---

# Count plugin updates during update checks

## Trigger

Panda Reader checks for app updates at start-up or every six hours while Check for updates automatically is on.

## Outcome

About shows "{} community plugin update(s) available. Open Plugins to update." Nothing is installed without the Reader.
