---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: Enter the feed URL and choose Add feed
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Show the download or parse error and add nothing
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
---

# The address does not return a readable feed

## Trigger

The download fails, exceeds 32 MiB, or the response is not a feed.

## Outcome

The error is shown and the library is unchanged.
