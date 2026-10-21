---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Disconnect
    kind: actor
    actor: reader
    entities:
      - entity: miniflux-connection
        effect: reads
        facts:
          - Server URL
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Remove the saved Miniflux connection
    kind: product
    actor: reader
    entities:
      - entity: miniflux-connection
        effect: removes
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Keep the cached Miniflux library available for reading
    kind: product
    actor: reader
    entities:
      - entity: library
        effect: reads
        facts:
          - Source
      - entity: article
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::article-list
---

# Disconnect and keep the cached library

## Trigger

The Reader no longer wants Panda Reader to reach their Miniflux server.

## Outcome

The connection is removed, "Disconnected; cached articles remain available" is shown, and the Miniflux library's articles can still be read.
