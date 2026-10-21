---
appliesTo:
  - type: capability
    id: add-feed
  - type: capability
    id: import-opml
references:
  - kind: code
    role: implementation
    target: crates/panda-store/src/store.rs#normalize_http_url
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/tests.rs#subscription_urls_only_allow_http_schemes
---

# Feeds are added only from HTTP or HTTPS addresses

Add feed refuses any other scheme with "Feed URL must use HTTP or HTTPS", and OPML import skips such outlines.
