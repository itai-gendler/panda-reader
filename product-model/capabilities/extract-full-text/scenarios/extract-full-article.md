---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Extract full text on the open article
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Link
    contexts:
      desktop:
        place: desktop::article
  - text: Pass the link through enabled URL resolver plugins that match it
    kind: product
    actor: reader
    entities:
      - entity: article-plugin
        effect: reads
        facts:
          - Domains
          - Stage
          - Network hosts
    contexts:
      desktop:
        place: desktop::article
  - text: Download the original page over the full-text fetch network
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Full-text fetch network
    contexts:
      desktop:
        place: desktop::article
  - text: Repair the page with matching prepare plugins and extract the article with the chosen content extractor
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Content extractor
      - entity: article-plugin
        effect: reads
        facts:
          - Domains
          - Stage
      - entity: article
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::article
  - text: Save the full text and the page and drop the translation made from the old body
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
        facts:
          - Full text
          - Original page
          - Link
          - Translation
          - Translated title
    contexts:
      desktop:
        place: desktop::article
  - text: Show the article with its full text
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Full text
    contexts:
      desktop:
        place: desktop::article
---

# Extract the full article

## Trigger

The open article only has a summary from its feed.

## Outcome

The article shows the full text from its original page, "Full text extracted" is shown, and the downloaded page is kept so the text can be extracted again offline.
