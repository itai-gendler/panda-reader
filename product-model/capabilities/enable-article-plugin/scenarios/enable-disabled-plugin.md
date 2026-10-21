---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Turn on the plugin's switch
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
  - text: Enable the plugin and reload the plugins
    kind: product
    actor: reader
    entities:
      - entity: article-plugin
        effect: changes
        from: Disabled
        to: Enabled
        facts: []
    contexts:
      desktop:
        place: desktop::plugin-settings
---

# Enable a plugin

## Trigger

The Reader wants a disabled plugin back.

## Outcome

The plugin runs on matching pages again, and the open article is prepared with it.
