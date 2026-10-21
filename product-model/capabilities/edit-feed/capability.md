---
availability:
  - place: desktop
domain: feed-management
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/feed_editor.rs#save_feed_editor
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/subscriptions.rs#update_feed
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/feeds.rs#update_feed
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/feeds.rs#set_feed_auto_translate_titles
---

# Edit feed

Change a feed's title, folder and feed URL, and whether its titles are translated, from **Edit** in Added feeds or **Edit feed…** in the feed's menu. Title, folder and URL are editable only in the Local library.
