---
availability:
  - place: desktop
domain: articles
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/article_view/actions.rs#schedule_reading_progress_save
  - kind: code
    role: implementation
    target: crates/panda-store/src/store.rs#save_reading_progress
---

# Save reading position

Remember how far the Reader scrolled in each article while Remember reading position is on, so the article reopens there.
