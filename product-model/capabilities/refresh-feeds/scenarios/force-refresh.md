---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Shift-click Refresh feeds
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::feeds
  - text: Download every Local feed without its cache validators
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Feed URL
    contexts:
      desktop:
        place: desktop::feeds
  - text: Save new entries as unread articles
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: creates
        to: Unread
        facts:
          - Title
          - Link
          - Author
          - Published
          - Summary
          - Feed content
    contexts:
      desktop:
        place: desktop::feeds
  - text: Update the saved entries whose content changed
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
        facts:
          - Title
          - Link
          - Author
          - Published
          - Summary
          - Feed content
    contexts:
      desktop:
        place: desktop::feeds
---

# Force a refresh

## Trigger

The Reader Shift-clicks Refresh feeds.

## Outcome

Every Local feed is downloaded in full whatever its cache validators say; in a Miniflux library the server is first asked to refresh all its feeds.

## Decision points

### Which library is selected?

What does a forced refresh ask of the source?

- Local subscriptions → every feed is downloaded without cache validators.
- Miniflux → the server is asked to refresh all feeds, then the library synchronizes.
- FreshRSS → the provider offers no refresh request, so the library synchronizes with what the server has.
