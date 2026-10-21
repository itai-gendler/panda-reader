---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Reload
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::plugin-settings
  - text: Load every installed plugin again and list the ones that loaded
    kind: product
    actor: reader
    entities:
      - entity: article-plugin
        effect: reads
        facts:
          - Name
          - Version
          - Last error
    contexts:
      desktop:
        place: desktop::plugin-settings
---

# Reload plugins after editing one

## Trigger

The Reader changed an installed plugin's files while testing it.

## Outcome

The plugins are read again (a plugin whose files no longer load drops out of the list) and the open article is prepared with the new plugins.
