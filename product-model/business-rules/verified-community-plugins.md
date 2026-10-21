---
appliesTo:
  - type: capability
    id: install-community-plugin
  - type: capability
    id: check-for-plugin-updates
references:
  - kind: code
    role: implementation
    target: crates/panda-plugins/src/catalog.rs#install_verified
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/worker.rs#install_community_plugin
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/plugins/install.md
---

# Community plugins are installed only when they match the catalog

Each community plugin file is downloaded over HTTPS without credentials, must match the SHA-256 hash the catalog lists, and its manifest must match the catalog entry. Plugins that need a newer app or another plugin API version are not installed. Updates are offered, never applied silently.
