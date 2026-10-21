---
availability:
  - place: desktop
domain: articles
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/article_view/actions.rs#open_article
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/articles/mod.rs#prepare_article
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/articles/mod.rs#canonical_article_body
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/article_list/actions.rs#move_article
---

# Open article

Open an article from the list, or move to the next or previous (unread) article, and read it. Opening an article marks it read.
