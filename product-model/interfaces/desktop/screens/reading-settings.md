---
entities:
  - entity: settings
    shows:
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
    collects:
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
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/reading.rs#render_reading_settings
---

# Reading

Typography, reading position, layout, keyboard, sharing and full-text settings.
