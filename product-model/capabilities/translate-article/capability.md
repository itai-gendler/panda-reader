---
availability:
  - place: desktop
domain: translation
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/articles/mod.rs#translate
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/article_view/actions.rs#toggle_or_translate
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/translation.rs#persist_translation
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/translation.rs#save_translation_segment
  - kind: code
    role: implementation
    target: crates/panda-translate/src/llm.rs
---

# Translate

Translate the open article's body and title into the article language with the configured translation provider, and show the translation in the chosen display mode. Once made, a translation is shown again without a new request (**Show translation**).
