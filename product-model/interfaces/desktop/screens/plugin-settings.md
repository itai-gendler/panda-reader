---
entities:
  - entity: article-plugin
    shows:
      - Name
      - ID
      - Version
      - Domains
      - Network hosts
      - Built-in
      - Last error
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/plugins.rs#render_plugins_settings
---

# Plugins

Installed article plugins with their switches, import from a folder, ZIP archive or HTTPS URL, and the community plugin list with Install and Update.
