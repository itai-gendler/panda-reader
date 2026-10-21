---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: Choose Extract full text
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Link
    contexts:
      desktop:
        place: desktop::article
  - text: Extract fewer than 80 characters of text from the page
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::article
  - text: Show "No extractable article content found on this page" and keep the feed content
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Feed content
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::article
---

# The page has no article text

## Trigger

The page yields under 80 characters of text, even after a prepare plugin selected the body.

## Outcome

"No extractable article content found on this page" is shown and the article keeps its feed content.
