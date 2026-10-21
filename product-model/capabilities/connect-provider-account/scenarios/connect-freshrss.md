---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Enter the Server URL, Username and API Password
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
  - text: >-
      Sign in to the Google Reader-compatible API at /api/greader.php, adding that path when the address lacks
      it
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Save the FreshRSS connection and report the username
    kind: product
    actor: reader
    entities:
      - entity: freshrss-connection
        effect: creates
        facts:
          - Server URL
          - Username
          - API password
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Start a sync of the FreshRSS library
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
    target: crates/panda-providers/src/freshrss.rs#FreshRssConnection
---

# Connect FreshRSS through the Google Reader API

## Trigger

The Reader has chosen FreshRSS and set an API password in their FreshRSS account settings.

## Outcome

The FreshRSS connection is saved for the server's /api/greader.php endpoint and a sync of the FreshRSS library starts.
