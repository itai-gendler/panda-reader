---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Local subscriptions under Mode / Provider
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: changes
        facts:
          - Subscription source
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Show the Local library's feeds and articles, which refresh straight from their RSS sources
    kind: product
    actor: reader
    entities:
      - entity: library
        effect: reads
        facts:
          - Source
      - entity: feed
        effect: reads
        facts:
          - Title
          - Unread count
      - entity: article
        effect: reads
        facts:
          - Title
          - Title translation
          - Published
    contexts:
      desktop:
        place: desktop::article-list
---

# Return to Local subscriptions

## Trigger

The Reader switches back from a provider library to Local subscriptions.

## Outcome

The Local library is shown with its own feeds and reading state; the provider library and its connection stay saved for the next switch.
