---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Share link on the open article
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
          - Link
    contexts:
      desktop:
        place: desktop::article
  - text: Fill the Share template with the title and link and copy it
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Share template
    contexts:
      desktop:
        place: desktop::article
---

# Copy the share text

## Trigger

The Reader wants to send an article to someone.

## Outcome

The clipboard holds the Share template with {title} and {url} filled in and \n turned into line breaks, and "Share text copied" is shown.

## Edge cases

- An empty Share template falls back to the title and link on two lines.
