---
appliesTo:
  - type: capability
    id: mark-all-as-read
  - type: capability
    id: browse-articles
references:
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/articles.rs#mark_all_read
---

# Read all marks only the articles in view

Mark all as read applies to the current view (All Articles and Unread cover the whole library; Starred, Read Later, a feed or a folder only their articles) and, when a search is typed, only to the articles that match it.
