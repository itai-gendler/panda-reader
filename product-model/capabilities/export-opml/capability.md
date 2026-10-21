---
availability:
  - place: desktop
domain: feed-management
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/subscriptions.rs#export_opml
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/subscriptions.rs#export_opml
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/opml.rs#export_opml
---

# Export OPML

Save the selected library's subscriptions as an OPML file.
