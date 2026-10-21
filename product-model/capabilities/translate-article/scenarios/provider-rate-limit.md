---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: Choose Translate on the open article
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::article
  - text: Report that the translation provider rate limit was reached
    kind: product
    actor: reader
    entities:
      - entity: translation-provider
        effect: reads
        facts:
          - Provider
    contexts:
      desktop:
        place: desktop::article
---

# The translation provider is rate limiting

## Trigger

The provider answers HTTP 429.

## Outcome

"Translation provider rate limit reached … Wait a moment before retrying." is shown and the article stays in its original language.
