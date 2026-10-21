---
appliesTo:
  - type: capability
    id: extract-full-text
  - type: capability
    id: change-developer-settings
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/articles/fetch.rs#article_redirect_url
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/articles/fetch.rs#is_local_or_private_ip
---

# Article pages are fetched only from public addresses

Full-text downloads go only to public network addresses, also after each redirect, and never downgrade from HTTPS to HTTP. Private and local addresses stay blocked in every mode; synthetic routing ranges are accepted only when they are configured and do not overlap private, local or multicast networks.
