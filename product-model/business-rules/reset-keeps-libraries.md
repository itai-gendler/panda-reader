---
appliesTo:
  - type: capability
    id: reset-settings
  - type: capability
    id: change-subscription-source
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/actions.rs#reset_settings
---

# Reset settings keeps libraries and provider connections

Restore defaults resets the interface, reading, translation (including saved translation keys), network and diagnostics settings and returns to Local subscriptions, but keeps every library's feeds and articles and the saved Miniflux and FreshRSS connections.
