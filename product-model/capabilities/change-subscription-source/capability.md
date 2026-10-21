---
availability:
  - place: desktop
domain: subscription-source
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/state.rs
  - kind: code
    role: implementation
    target: apps/panda-reader/src/app/preferences.rs#LibrarySource
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/providers.md
---

# Change subscription source

Choose under **Mode / Provider** whether Panda Reader shows the Local library, the Miniflux library or the FreshRSS library. The other libraries are kept as they are.
