---
appliesTo:
  - type: capability
    id: edit-feed
  - type: capability
    id: refresh-feeds
  - type: entity
    id: feed
    facts:
      - Title
      - Folder
references:
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/feeds.rs#update_feed
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/feeds.rs#write_prepared_feed
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/tests.rs#edited_feed_fields_and_translation_preference_survive_refresh
---

# A Local feed keeps its edited title and folder through refreshes

A refresh updates the title the feed publishes but the title the Reader gave it stays shown, and the folder is never changed by a refresh.
