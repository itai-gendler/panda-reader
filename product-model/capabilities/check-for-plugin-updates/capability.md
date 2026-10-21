---
availability:
  - place: desktop
domain: plugins
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/plugins.rs#check_community_plugins
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/worker.rs#community_plugin_catalog
  - kind: code
    role: implementation
    target: crates/panda-plugins/src/catalog.rs
---

# Check for plugin updates

Fetch the community catalog to see which plugins can be installed or updated, from **Check for updates** under Community plugins, and automatically with app update checks.
