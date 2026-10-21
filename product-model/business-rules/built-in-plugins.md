---
appliesTo:
  - type: capability
    id: remove-article-plugin
  - type: capability
    id: import-article-plugin
  - type: entity
    id: article-plugin
    facts:
      - Built-in
references:
  - kind: code
    role: implementation
    target: crates/panda-plugins/src/registry.rs#install_directory
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/plugins.rs#plugin_row
---

# Only plugins the Reader installed can be removed

The plugin that ships with Panda Reader can be disabled but not removed, and no imported plugin may use an ID in the reserved builtin. namespace.
