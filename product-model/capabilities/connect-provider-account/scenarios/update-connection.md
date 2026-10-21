---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Connected shows the saved server
    kind: condition
    entities:
      - entity: miniflux-connection
        effect: reads
        facts:
          - Server URL
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Change the API Token and choose Update connection
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Check the new details with the server and save them
    kind: product
    actor: reader
    entities:
      - entity: miniflux-connection
        effect: changes
        facts:
          - Server URL
          - API token
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Start a sync of the Miniflux library
    kind: product
    actor: reader
    entities:
      - entity: library
        effect: reads
        facts:
          - Source
    contexts:
      desktop:
        place: desktop::general-settings
---

# Update the connection

## Trigger

A Miniflux connection is saved and the Reader changes its server or token.

## Outcome

The changed connection is saved and a sync starts. When it now reaches a different account, that sync replaces the cached library once it completes.
