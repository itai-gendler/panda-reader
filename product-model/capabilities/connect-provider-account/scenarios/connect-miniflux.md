---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Enter the Server URL and API Token
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Choose Connect and sync
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Ask the Miniflux server who the token belongs to
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Save the Miniflux connection and report the account name
    kind: product
    actor: reader
    entities:
      - entity: miniflux-connection
        effect: creates
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
references:
  - kind: code
    role: implementation
    target: crates/panda-miniflux/src/lib.rs#me
---

# Connect Miniflux and sync

## Trigger

The Reader has chosen Miniflux as the subscription source and has an API token from their Miniflux instance.

## Outcome

The Miniflux connection is saved, "Connected:" shows the account name, and Panda Reader starts a sync of the Miniflux library.
