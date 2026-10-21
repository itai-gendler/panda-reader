---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Paste the HTTPS link to the ZIP archive and choose Import
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::plugin-settings
  - text: Download the archive over HTTPS, refusing redirects to other schemes
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::plugin-settings
  - text: Check the archive and install the plugin
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

# Import a plugin from an HTTPS link

## Trigger

A plugin author publishes a ZIP archive.

## Outcome

The archive is downloaded over HTTPS (at most 8 MiB), checked and installed like a folder.
