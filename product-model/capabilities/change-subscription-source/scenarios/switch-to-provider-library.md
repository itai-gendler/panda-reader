---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Miniflux under Mode / Provider
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
  - text: Stop work on the open article, save the choice and show the saved Miniflux connection details
    kind: product
    actor: reader
    entities:
      - entity: miniflux-connection
        effect: reads
        facts:
          - Server URL
          - API token
      - entity: article
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Load the Miniflux library's feeds and its newest articles under All Articles
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

# Switch to a provider library

## Trigger

The Reader wants to read from their Miniflux account instead of the Local library.

## Outcome

Settings now show Miniflux as the subscription source and the article list shows the Miniflux library from All Articles. The Local library keeps its feeds, articles and reading state.
