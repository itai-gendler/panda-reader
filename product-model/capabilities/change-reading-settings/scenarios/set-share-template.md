---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Edit the Share template using {title}, {url} and \n
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Share template
    contexts:
      desktop:
        place: desktop::reading-settings
  - text: Save the template
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: changes
        facts:
          - Share template
    contexts:
      desktop:
        place: desktop::reading-settings
---

# Set the share template

## Trigger

The Reader wants shared links in their own format.

## Outcome

Share link copies text in the new template.
