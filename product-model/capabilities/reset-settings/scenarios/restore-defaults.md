---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Reset settings
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Choose Restore defaults
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Restore every setting and the translation provider to its default, keeping libraries and connections
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: changes
        facts:
          - Language
          - Subscription source
          - Article font
          - Text size
          - Line height
          - Text width
          - Paragraph spacing
          - Remember reading position
          - Collapse feed sidebar
          - First-line indent
          - Vim navigation
          - Share template
          - Auto extract full text
          - Content extractor
          - Hide images
          - Theme
          - Interface size
          - Show panda icon
          - Article language
          - Automatically translate titles
          - Only translate future titles
          - Display mode
          - Check for updates automatically
          - Detailed sync logging
          - Log retention
          - Full-text fetch network
          - Proxy URL
          - Synthetic routing CIDR ranges
          - Last refresh
      - entity: translation-provider
        effect: changes
        facts:
          - Provider
          - Azure API key
          - Azure region
          - Access Key ID
          - Secret Access Key
          - API URL
          - API key
          - Model
      - entity: library
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Show the Local library from All Articles
    kind: product
    actor: reader
    entities:
      - entity: library
        effect: reads
        facts:
          - Source
      - entity: article
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::article-list
---

# Restore the default settings

## Trigger

The Reader has changed many settings and wants a clean start.

## Outcome

"Settings restored to defaults." is shown. Interface, reading, translation (including saved translation keys), network and diagnostics settings are back to their defaults and the Local library is shown; feeds, articles and provider connections are kept.
