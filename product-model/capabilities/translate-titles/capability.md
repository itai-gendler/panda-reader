---
availability:
  - place: desktop
domain: translation
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/article_list/actions.rs#queue_title_translations
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/articles/mod.rs#translate_titles
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/translation.rs#save_auto_translated_title
---

# Translate titles

Translate the titles of listed articles in the background while **Automatically translate titles** is on, for feeds marked Translate titles. The original title shows until its translation arrives.
