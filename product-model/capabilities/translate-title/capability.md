---
availability:
  - place: desktop
domain: translation
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/article_list/actions.rs#translate_title_manually
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/articles/mod.rs#translate_titles
---

# Translate title

Translate one article's title for the list from its menu, whatever the feed's Translate titles setting.
