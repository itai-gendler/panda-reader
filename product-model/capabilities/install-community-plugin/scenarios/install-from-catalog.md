---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Install on the plugin in Community plugins
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::plugin-settings
  - text: Download the plugin's files over HTTPS and check their SHA-256 hashes and manifest against the catalog
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::plugin-settings
  - text: Install the verified plugin
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
---

# Install a community plugin

## Trigger

The Reader reads a site the community catalog has a plugin for.

## Outcome

The plugin is installed, enabled and its row shows Installed.

## Edge cases

- After every plugin change the open article is prepared again with the new set of plugins.
