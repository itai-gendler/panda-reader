---
appliesTo:
  - type: capability
    id: extract-full-text
  - type: capability
    id: open-article
  - type: entity
    id: article-plugin
    facts:
      - Domains
      - Path prefixes
      - Priority
      - Stage
references:
  - kind: code
    role: implementation
    target: crates/panda-plugins/src/registry.rs#process
  - kind: code
    role: implementation
    target: crates/panda-plugins/src/registry.rs#load
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/plugins/manifest.md
---

# Plugins run only on matching pages, highest priority first

An enabled plugin runs at its stage only on pages whose host matches one of its domains and, when it lists path prefixes, whose path starts with one. Plugins run from the highest priority, then by ID, each on the previous one's result.
