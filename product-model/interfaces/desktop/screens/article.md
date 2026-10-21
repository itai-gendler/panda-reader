---
entities:
  - entity: article
    shows:
      - Title
      - Author
      - Published
      - Link
      - Feed content
      - Full text
      - Translation
      - Translated title
      - Starred
      - Read later
      - Reading position
  - entity: feed
    shows:
      - Title
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/article_view/view.rs
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/article_view/actions.rs#open_article
---

# Article

The open article: its feed, author, date and title, and its body as feed content, full text or translation, with the reading actions.
