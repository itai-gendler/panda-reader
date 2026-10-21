---
entities:
  - entity: feed
    shows:
      - Title
      - Folder
      - Feed URL
      - Translate titles
    collects:
      - Title
      - Folder
      - Feed URL
      - Translate titles
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/feed_editor.rs#render_feed_editor
---

# Edit feed

Change one feed: its title, folder and feed URL in the Local library, and in every library whether its titles are translated.
