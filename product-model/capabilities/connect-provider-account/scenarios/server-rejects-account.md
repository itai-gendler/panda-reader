---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: Choose Connect and sync
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Show the server error and keep the saved connection unchanged
    kind: product
    actor: reader
    entities:
      - entity: miniflux-connection
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
---

# The server rejects the account

## Trigger

The server is unreachable or does not accept the credentials.

## Outcome

The error is shown and any previously saved connection stays as it was.
