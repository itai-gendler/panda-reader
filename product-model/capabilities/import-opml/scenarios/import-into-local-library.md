---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Import OPML and pick an .opml or .xml file
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Read the feed outlines, taking a parent outline's text as the folder
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Skip addresses that are not HTTP or HTTPS or already in the library
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Feed URL
      - entity: library
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Download the remaining feeds, eight at a time
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Add each feed that answered with its outline title and folder, and its entries as unread articles
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: creates
        to: Active
        facts:
          - Title
          - Feed URL
          - Site URL
          - Folder
      - entity: article
        effect: creates
        to: Unread
        facts:
          - Title
          - Link
          - Author
          - Published
          - Summary
          - Feed content
        with: feed
    contexts:
      desktop:
        place: desktop::general-settings
---

# Import an OPML file into the Local library

## Trigger

The Reader moves to Panda Reader with an OPML export from another reader.

## Outcome

Each new, readable feed is added with the title and folder its outline gives, and "Imported {} feeds" reports how many.

## Edge cases

- A feed that cannot be downloaded is skipped without stopping the import.
