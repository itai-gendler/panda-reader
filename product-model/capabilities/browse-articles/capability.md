---
availability:
  - place: desktop
domain: articles
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/article_list/actions.rs#select_scope
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/article_list/actions.rs#load_snapshot_page
  - kind: code
    role: implementation
    target: crates/panda-store/src/store.rs#snapshot
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/fts.rs#fts_query
---

# Browse articles

List the articles of All Articles, Unread, Starred, Read Later, one feed or one folder, newest first and 20 at a time, optionally narrowed by **Search articles**.
