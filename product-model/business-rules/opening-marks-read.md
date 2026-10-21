---
appliesTo:
  - type: capability
    id: open-article
  - type: entity
    id: article
    effect: changes
    to: Read
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/article_view/actions.rs#open_article
---

# Opening an article marks it read

An article becomes read as soon as it is shown, whether it was chosen in the list or reached with Next article or Next unread.
