---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Larger under Interface size
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Interface size
    contexts:
      desktop:
        place: desktop::appearance-settings
  - text: Save the larger size
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: changes
        facts:
          - Interface size
    contexts:
      desktop:
        place: desktop::appearance-settings
---

# Change the interface size

## Trigger

The sidebar and list text are hard to read.

## Outcome

Text and controls in the sidebar, list and settings grow, up to 18 px.
