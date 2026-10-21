---
availability:
  - place: desktop
domain: plugins
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/worker.rs#remove_plugin
  - kind: code
    role: implementation
    target: crates/panda-plugins/src/registry.rs#remove_user_plugin
---

# Remove article plugin

Delete an installed plugin. Only plugins the Reader installed have a Remove button.
