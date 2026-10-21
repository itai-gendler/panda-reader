---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Cancel while the article is translating
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
  - text: Stop the translation request
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::article
---

# Cancel a running translation

## Trigger

A long article is being translated and the Reader changes their mind.

## Outcome

"Translation cancelled" is shown. With a language-model provider, paragraphs already finished stay saved for the next attempt.

## Edge cases

- Opening another article or switching the subscription source also stops the open article's translation.
