---
availability:
  - place: desktop
domain: plugins
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/worker.rs#reload_plugins
  - kind: code
    role: implementation
    target: crates/panda-plugins/src/registry.rs#load
---

# Reload plugins

Load the installed plugins again from the data folder with **Reload**.
