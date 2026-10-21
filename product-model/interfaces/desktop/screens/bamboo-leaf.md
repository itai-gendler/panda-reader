---
entities:
  - entity: article
    shows:
      - Title
      - Link
  - entity: feed
    shows:
      - Title
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/quote_poster.rs#render_quote_poster
---

# Bamboo Leaf

A quote image made from text selected in an article, with the article's title, its feed and a QR code of its link, ready to copy or save as PNG.
