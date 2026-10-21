---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose a feed marked Translate titles
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
  - text: Automatically translate titles is on and the feed is marked Translate titles
    kind: condition
    entities:
      - entity: settings
        effect: reads
        facts:
          - Automatically translate titles
          - Article language
      - entity: feed
        effect: reads
        facts:
          - Translate titles
    contexts:
      desktop:
        place: desktop::article-list
  - text: Translate the listed titles that have no translation in the article language, in one batch
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
          - Title translation
      - entity: translation-provider
        effect: reads
        facts:
          - Provider
    contexts:
      desktop:
        place: desktop::article-list
  - text: Save and show each title translation
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
        facts:
          - Title translation
    contexts:
      desktop:
        place: desktop::article-list
---

# Translate the titles in the list

## Trigger

Articles from a feed marked Translate titles are loaded into the list while Automatically translate titles is on.

## Outcome

The list replaces their titles with translations in the article language as they arrive, and "Translated {} title(s)" reports how many.
