---
kind: validation
routes:
  desktop: Desktop
steps:
  - text: Choose Connect and sync with an empty API Token
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Refuse with "Enter a Miniflux API token" without contacting the server
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
---

# Refuse incomplete connection details

## Trigger

The Reader chooses Connect and sync with an empty API token, or a server address that is not HTTP or HTTPS.

## Outcome

An error such as "Enter a Miniflux API token" or "Miniflux URL must use http or https" is shown and nothing is saved.
