---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Remove star in the article's menu while Starred is shown
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::article-list
  - text: Remove the star and take the article out of the Starred view
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
        facts:
          - Starred
    contexts:
      desktop:
        place: desktop::article-list
---

# Remove a star in the Starred view

## Trigger

The Reader no longer needs an article they starred.

## Outcome

The article is no longer starred and leaves the Starred view; in a provider library the change is sent to the provider.
