---
availability:
  - place: desktop
domain: subscription-source
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/connections.rs#disconnect
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/subscriptions.rs#disconnect_provider
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/providers.md
---

# Disconnect provider account

Forget the saved server and credentials of the selected provider. Its cached library stays readable; changes that need the server wait until it is connected again.
