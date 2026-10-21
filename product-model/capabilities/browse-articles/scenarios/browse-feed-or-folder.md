---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose a folder in the feed column
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Folder
    contexts:
      desktop:
        place: desktop::feeds
  - text: List the articles of the feeds filed under that folder, newest first
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
          - Title translation
          - Published
      - entity: feed
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::article-list
---

# Browse a feed or a folder

## Trigger

The Reader chooses a feed, or a folder, in the feed column.

## Outcome

The list holds only that feed's articles, or the articles of every feed in that folder.
