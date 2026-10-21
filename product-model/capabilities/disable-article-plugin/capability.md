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

# Disable article plugin

Turn off a plugin's switch so it stops running without being removed. The built-in plugin can be disabled too.
