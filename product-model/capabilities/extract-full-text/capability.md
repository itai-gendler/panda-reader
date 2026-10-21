---
availability:
  - place: desktop
domain: articles
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/articles/mod.rs#extract
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/article_view/actions.rs#extract
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/articles/fetch.rs#fetch_html_with_network
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/extraction.rs#prepare_extraction_from_page
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/extraction.rs#persist_extraction
  - kind: code
    role: implementation
    target: crates/panda-content/src/extraction.rs
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/plugins/index.md
---

# Extract full text

Replace a short feed entry with the article from its original page: **Extract full text**, **Re-extract full text** or **Re-fetch current article** (Shift+Mod+R), or automatically on opening while Auto extract full text is on.
