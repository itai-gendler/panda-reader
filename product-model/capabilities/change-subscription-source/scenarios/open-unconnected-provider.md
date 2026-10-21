---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: Choose FreshRSS under Mode / Provider
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
  - text: No FreshRSS connection is saved and the FreshRSS library has no articles
    kind: condition
    entities:
      - entity: freshrss-connection
        effect: reads
        facts: []
      - entity: library
        effect: reads
        facts:
          - Source
      - entity: article
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::article-list
  - text: Show "No articles yet" with a prompt to connect the selected provider in Settings
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts: []
      - entity: settings
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::article-list
---

# Open a provider library that is not connected

## Trigger

The Reader chooses FreshRSS before any FreshRSS connection has been saved.

## Outcome

The FreshRSS library opens with whatever it cached before, and with nothing cached the list says "No articles yet" and asks to connect the selected provider in Settings.
