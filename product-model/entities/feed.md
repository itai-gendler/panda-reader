---
domain: feed-management
relations:
  - entity: article
    verb: contains
    cardinality: one-to-many
references:
  - kind: code
    role: implementation
    target: crates/panda-core/src/model.rs#Feed
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/feeds.rs#update_feed
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/feeds.rs#write_prepared_feed
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/provider.rs#save_remote_feeds
---

# Feed

One subscription in a library. Local feeds are fetched by Panda Reader; provider feeds mirror the provider's subscriptions.

## Information kept

- **Title** — the name shown for the feed: the title it publishes, or in the Local library the title the Reader gave it
- **Feed URL** — the RSS or Atom address; unique within its library
- **Site URL** — the website the feed belongs to, used for its icon
- **Folder** — the folder the feed is filed under; a provider's category becomes the folder
- **Unread count** — how many of its articles are unread
- **Refresh error** — the error of the last failed refresh of a Local feed
- **Translate titles** — whether titles from this feed are translated automatically
- **Titles translated from** — the newest article when Translate titles was turned on; with Only translate future titles, only later articles are translated automatically
- **Language** — the language a Miniflux feed declares

## States

### Active

The feed refreshed successfully, or has not failed since it was added.

### Refresh failed

The last refresh of this Local feed failed; the feed column shows the error until a refresh succeeds.
