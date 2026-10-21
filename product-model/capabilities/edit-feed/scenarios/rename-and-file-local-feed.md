---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Edit feed… in the feed's menu
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::feeds
  - text: Change the Title and Folder and choose Save
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Title
          - Folder
    contexts:
      desktop:
        place: desktop::edit-feed
  - text: Save the title, folder and feed URL
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: changes
        facts:
          - Title
          - Folder
          - Feed URL
    contexts:
      desktop:
        place: desktop::edit-feed
---

# Rename a Local feed and file it in a folder

## Trigger

The Reader wants a Local feed under a shorter name in a folder.

## Outcome

The feed shows the new title under the folder, "Feed updated" is shown, and the title is kept when the feed next refreshes.
