---
appliesTo:
  - type: capability
    id: extract-full-text
  - type: capability
    id: open-article
  - type: entity
    id: article-plugin
    facts:
      - Last error
references:
  - kind: code
    role: implementation
    target: crates/panda-plugins/src/registry.rs#process
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/articles/mod.rs#extract
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/plugins/security.md
---

# A failing plugin changes nothing

If a plugin fails, its changes are discarded, its error is shown with it under Plugins, and the remaining plugins and the extraction continue. Only a failing URL resolver stops a full-text extraction, since the page to fetch is then unknown.
