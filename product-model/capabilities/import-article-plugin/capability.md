---
availability:
  - place: desktop
domain: plugins
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/worker.rs#import_plugin
  - kind: code
    role: implementation
    target: crates/panda-plugins/src/registry.rs#install_directory
  - kind: code
    role: implementation
    target: crates/panda-plugins/src/registry.rs#install_archive
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/plugins/install.md
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/plugins/manifest.md
---

# Import article plugin

Install an article plugin from a folder, a ZIP archive or an HTTPS link to a ZIP archive. Panda Reader validates the plugin and copies it into its own data folder.
