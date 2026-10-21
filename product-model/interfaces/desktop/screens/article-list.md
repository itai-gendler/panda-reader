---
entities:
  - entity: article
    shows:
      - Title
      - Title translation
      - Published
  - entity: feed
    shows:
      - Title
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/article_list/view.rs
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/context_menus.rs#article_context_menu
---

# Article list

The articles of the chosen view, feed or folder, newest first, with search and Read all. Each row shows whether it is read, its title (or its title translation), its feed and its date.
