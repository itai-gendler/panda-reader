---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Turn on Vim navigation
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Vim navigation
    contexts:
      desktop:
        place: desktop::reading-settings
  - text: Save the choice
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: changes
        facts:
          - Vim navigation
    contexts:
      desktop:
        place: desktop::reading-settings
---

# Turn on Vim navigation

## Trigger

The Reader prefers single-key navigation.

## Outcome

Keys such as j, k, s, u and l move and act on articles when no text field has focus.
