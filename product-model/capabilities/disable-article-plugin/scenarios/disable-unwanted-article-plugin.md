---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Turn off the plugin's switch
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
  - text: Disable the plugin and reload the plugins
    kind: product
    actor: reader
    entities:
      - entity: article-plugin
        effect: changes
        from: Enabled
        to: Disabled
        facts: []
    contexts:
      desktop:
        place: desktop::plugin-settings
---

# Disable a plugin

## Trigger

A plugin cleans a site in a way the Reader does not want.

## Outcome

The plugin stays installed but no longer runs, and the open article is prepared without it.
