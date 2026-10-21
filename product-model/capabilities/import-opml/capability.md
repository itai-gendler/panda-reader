---
availability:
  - place: desktop
domain: feed-management
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/subscriptions.rs#import_opml
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/subscriptions.rs#import_opml
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/feeds.rs#parse_opml_feeds
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/feeds.rs#persist_imported_feed_response
---

# Import OPML

Add the subscriptions listed in an OPML file to the selected library.
