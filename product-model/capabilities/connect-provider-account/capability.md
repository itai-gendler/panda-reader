---
availability:
  - place: desktop
domain: subscription-source
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/connections.rs#connect
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/subscriptions.rs#connect_provider
  - kind: code
    role: implementation
    target: crates/panda-providers/src/lib.rs#identity
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/providers.md
---

# Connect provider account

Save the server and credentials of the selected provider with **Connect and sync**, or change them with **Update connection**. Panda Reader checks the account with the server before saving anything, then synchronizes the library.
