---
entities:
  - entity: feed
    shows:
      - Title
      - Folder
      - Unread count
      - Refresh error
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/sidebar/view.rs
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/context_menus.rs#feed_context_menu
---

# Feeds

The feed column: the All Articles, Unread, Starred and Read Later views, then the library's folders and feeds with their unread counts and any refresh error. Choosing one fills the article list. Each feed has a menu to refresh it, mark it as read or remove it, and in the Local library to edit it.
