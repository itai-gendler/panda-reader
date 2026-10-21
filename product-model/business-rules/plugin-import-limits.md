---
appliesTo:
  - type: capability
    id: import-article-plugin
  - type: entity
    id: article-plugin
    effect: creates
    to: Enabled
references:
  - kind: code
    role: implementation
    target: crates/panda-plugins/src/registry.rs#install_archive
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/worker.rs#download_plugin_archive
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/plugins/install.md
---

# Plugin imports are bounded

An imported ZIP archive may hold one plugin, at most 64 entries and 8 MiB; a link must be HTTPS without credentials and may only redirect to HTTPS. The manifest must declare the plugin API version, a name, a version and at least one domain, and the payload must match its format.
