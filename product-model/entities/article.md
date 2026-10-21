---
domain: articles
references:
  - kind: code
    role: implementation
    target: crates/panda-core/src/model.rs#Article
  - kind: code
    role: implementation
    target: crates/panda-core/src/model.rs#ArticleSummary
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/articles.rs#mark
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/extraction.rs#persist_extraction
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/translation.rs#persist_translation
---

# Article

One entry from a feed, kept on this computer for offline reading together with its reading state, full text and translations.

## Information kept

- **Title** — the entry title
- **Link** — the address of the original page; full-text extraction can replace it with the resolved address
- **Author** — the entry author
- **Published** — the publication or update date; articles are listed newest first
- **Summary** — a plain-text summary of the entry
- **Feed content** — the cleaned HTML body the feed or provider supplied
- **Original page** — the downloaded original web page, kept so full text can be extracted again offline
- **Full text** — the article body extracted from the original page
- **Starred** — whether the Reader starred it
- **Read later** — whether the Reader saved it for later
- **Translation** — the translated body and its language, for the article content and translation provider it was made from
- **Translated title** — the title translated together with the body
- **Title translation** — the title translated for the article list, with its language
- **Reading position** — how far the Reader had scrolled, from 0 to 1

## States

### Unread

The article has not been opened or was marked unread.

### Read

The article was opened or marked as read, here or at the provider.
