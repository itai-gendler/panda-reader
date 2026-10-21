---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Export OPML
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Build the OPML from the library's feeds, grouped by folder
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Title
          - Feed URL
          - Folder
      - entity: library
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Choose where to save the file
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
---

# Export the Local library

## Trigger

The Reader wants a copy of their subscriptions.

## Outcome

An OPML file titled "Panda Reader subscriptions", grouped by folder, is saved where the Reader chose (panda-reader.opml by default) and "OPML exported" is shown.
