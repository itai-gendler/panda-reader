---
appliesTo:
  - type: capability
    id: translate-titles
  - type: capability
    id: edit-feed
  - type: entity
    id: feed
    facts:
      - Translate titles
      - Titles translated from
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/article_list/actions.rs#queue_title_translations
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/feeds.rs#set_feed_auto_translate_titles
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/tests.rs#feed_title_translation_is_manual_and_persisted
---

# Automatic title translation covers only marked feeds

Titles are translated in the background only while Automatically translate titles is on, only for feeds marked Translate titles, and with Only translate future titles only for articles newer than the point where the feed was marked. Translate title in an article's menu works for any article.
