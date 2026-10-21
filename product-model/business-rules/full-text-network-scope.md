---
appliesTo:
  - type: capability
    id: extract-full-text
  - type: capability
    id: change-developer-settings
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/articles/fetch.rs#fetch_html_with_network
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/developer.rs
---

# The full-text fetch network applies only to article pages

The proxy mode under Advanced settings is used only to download original article pages for full text. RSS refresh, provider sync, plugin downloads and update checks are not affected.
