---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: Choose Sync now in the feed's menu
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::feeds
  - text: Record the refresh error on the feed and show it
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: changes
        from: Active
        to: Refresh failed
        facts:
          - Refresh error
    contexts:
      desktop:
        place: desktop::feeds
---

# The feed fails to refresh

## Trigger

The feed's server is unreachable or returns something that is not a feed.

## Outcome

The error is shown and kept on the feed in the feed column.
