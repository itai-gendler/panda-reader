---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Open original on the open article
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
  - text: Open the link in the system browser
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::article
---

# Open the original page

## Trigger

The Reader wants the publisher's page itself.

## Outcome

The system browser opens the article's link.
