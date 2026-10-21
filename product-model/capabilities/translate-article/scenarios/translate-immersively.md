---
kind: primary
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
  - text: A translation provider is configured
    kind: condition
    entities:
      - entity: translation-provider
        effect: reads
        facts:
          - Provider
    contexts:
      desktop:
        place: desktop::article
  - text: Translate the article's current body and title into the article language
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Article language
          - Display mode
      - entity: article
        effect: reads
        facts:
          - Feed content
          - Full text
          - Title
    contexts:
      desktop:
        place: desktop::article
  - text: Save the translation and, for a feed marked Translate titles, its title for the list
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
        facts:
          - Translation
          - Translated title
          - Title translation
      - entity: feed
        effect: reads
        facts:
          - Translate titles
    contexts:
      desktop:
        place: desktop::article
  - text: Show the translation in the display mode
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Translation
          - Translated title
    contexts:
      desktop:
        place: desktop::article
---

# Translate an article

## Trigger

The Reader opens an article in a language they read less easily, with a translation provider configured.

## Outcome

Each paragraph is followed by its translation (Immersive) or replaced by it (Translation only), and "Translation ready · …" gives the time taken. The translation is saved, the request is counted in daily usage, and when the feed is marked Translate titles the list shows the translated title too.

## Decision points

### Which display mode is chosen?

How is the translation shown?

- Immersive → each original paragraph is followed by its translation.
- Translation only → the translation replaces the original text.
