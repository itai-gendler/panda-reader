---
appliesTo:
  - type: capability
    id: extract-full-text
  - type: capability
    id: refresh-feeds
  - type: capability
    id: translate-article
  - type: entity
    id: article
    facts:
      - Full text
      - Translation
references:
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/extraction.rs#persist_extraction
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/provider.rs#save_prepared_remote_entries
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/translation.rs#persist_translation
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/tests.rs#stale_extraction_is_discarded_after_article_source_changes
---

# New content discards what was derived from the old

Extracting full text clears the article's translation. When a provider sends changed content for an article, its full text, translations and title translation are cleared too. Work that started on older content is not saved over newer content.
