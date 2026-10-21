---
type: desktop-app
actors:
  - reader
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/window.rs#ReaderWindow
  - kind: code
    role: implementation
    target: apps/panda-reader/src/app/mod.rs#run
  - kind: doc
    role: intent
    target: README.md
---

# Panda Reader desktop

The desktop window where the person on this computer browses feeds and articles, reads, translates and configures Panda Reader. Feed sites, Miniflux and FreshRSS servers, translation services, the community plugin catalog and the release server are outbound dependencies.
