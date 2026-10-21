---
availability:
  - place: desktop
domain: plugins
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/worker.rs#install_community_plugin
  - kind: code
    role: implementation
    target: crates/panda-plugins/src/catalog.rs#install_verified
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/plugins.rs#community_plugin_row
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/plugins/install.md
---

# Install community plugin

Install, or update, a plugin listed in the community catalog under **Community plugins**.
