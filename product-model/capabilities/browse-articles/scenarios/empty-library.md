---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: Choose a view in the feed column
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::feeds
  - text: The selected library has no articles
    kind: condition
    entities:
      - entity: article
        effect: reads
        facts: []
      - entity: library
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::article-list
  - text: Show "No articles yet"
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::article-list
---

# Nothing to show yet

## Trigger

The view has no articles, for example in a provider library that is not connected.

## Outcome

"No articles yet" is shown with a prompt to connect the selected provider in Settings.
