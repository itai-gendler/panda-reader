---
availability:
  - place: desktop
domain: plugins
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/worker.rs#set_plugin_enabled
  - kind: code
    role: implementation
    target: crates/panda-plugins/src/registry.rs#set_enabled
---

# Enable article plugin

Turn on a disabled plugin's switch so it runs on matching pages again.
