---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Remove on the plugin
    kind: actor
    actor: reader
    entities:
      - entity: article-plugin
        effect: reads
        facts:
          - Name
          - Built-in
    contexts:
      desktop:
        place: desktop::plugin-settings
  - text: Delete the plugin and reload the plugins
    kind: product
    actor: reader
    entities:
      - entity: article-plugin
        effect: removes
        from: Enabled
    contexts:
      desktop:
        place: desktop::plugin-settings
---

# Remove an installed plugin

## Trigger

The Reader no longer needs a plugin they installed.

## Outcome

The plugin's files and its on/off choice are deleted and it disappears from the list.
