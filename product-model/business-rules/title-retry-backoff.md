---
appliesTo:
  - type: capability
    id: translate-titles
  - type: entity
    id: article
    facts:
      - Title translation
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/articles/mod.rs#translate_titles
---

# A title is sent for automatic translation at most once a minute

A title already sent within the last minute for the same language and provider is deferred, whether or not the attempt succeeded. A title with a saved translation for the article language is not sent again.
