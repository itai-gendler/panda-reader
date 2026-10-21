---
kind: validation
routes:
  desktop: Desktop
steps:
  - text: 'Enter a file: address and choose Add feed'
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Refuse with "Feed URL must use HTTP or HTTPS"
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
references:
  - kind: code
    role: implementation
    target: crates/panda-store/src/store.rs#normalize_http_url
---

# Refuse an address that is not HTTP or HTTPS

## Trigger

The Reader enters an address such as a file: or ftp: URL.

## Outcome

"Feed URL must use HTTP or HTTPS" is shown and nothing is added.
