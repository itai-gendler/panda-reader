---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Browse folder, pick the plugin folder and choose Import
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::plugin-settings
  - text: Validate the manifest and payload and copy the plugin into the app's data
    kind: product
    actor: reader
    entities:
      - entity: article-plugin
        effect: creates
        to: Enabled
        facts:
          - ID
          - Name
          - Version
          - Format
          - Stage
          - Domains
          - Path prefixes
          - Priority
          - Host capabilities
          - Network hosts
          - Built-in
    contexts:
      desktop:
        place: desktop::plugin-settings
  - text: Reload the plugins and list them
    kind: product
    actor: reader
    entities:
      - entity: article-plugin
        effect: reads
        facts:
          - Name
          - ID
          - Version
          - Domains
    contexts:
      desktop:
        place: desktop::plugin-settings
---

# Import a plugin folder

## Trigger

The Reader has written a rules plugin in a folder with manifest.toml and rules.toml.

## Outcome

The plugin is copied into Panda Reader's data, enabled and listed; the original folder is not used afterwards.

## Edge cases

- After every plugin change the open article is prepared again with the new set of plugins.
