---
availability:
  - place: desktop
domain: articles
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/article_view/actions.rs#mark_all_read
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/articles/mod.rs#mark_all_read
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/articles.rs#mark_all_read
---

# Mark all as read

Mark every unread article in the current view as read with **Read all** above the list (Shift+X), or a whole feed with **Mark all as read** in its menu.
