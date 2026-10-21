---
domain: subscription-source
relations:
  - entity: library
    verb: syncs
    cardinality: one-to-one
references:
  - kind: code
    role: implementation
    target: crates/panda-providers/src/freshrss.rs#FreshRssConnection
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/providers.md
---

# FreshRSS connection

The saved FreshRSS server, username and API password the FreshRSS library synchronizes with through the Google Reader-compatible API.

## Information kept

- **Server URL** — the FreshRSS API endpoint; Panda Reader adds /api/greader.php when the address does not already end with it
- **Username** — the FreshRSS username
- **API password** — the API password set in the FreshRSS account settings
