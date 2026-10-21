---
appliesTo:
  - type: capability
    id: translate-article
  - type: capability
    id: show-original
  - type: entity
    id: article
    facts:
      - Translation
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/articles/mod.rs#translate
  - kind: code
    role: implementation
    target: crates/panda-translate/src/lib.rs#revision_cache_id
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/tests.rs#translation_segment_cache_is_scoped_by_article_language_backend_and_prompt
---

# A translation is reused until its source changes

A saved translation is shown again without a request as long as the article's body, the article language and the translation provider (and, for language-model providers, the API address and model) are unchanged. Language-model translations also keep finished paragraphs, so an interrupted translation continues where it stopped.
