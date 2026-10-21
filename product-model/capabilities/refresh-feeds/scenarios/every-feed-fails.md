---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: Choose Refresh feeds
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::feeds
  - text: Record the error on every feed and report "All feeds failed to refresh"
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

# Every feed fails

## Trigger

No feed of the Local library could be refreshed, for example while offline.

## Outcome

"All feeds failed to refresh:" is shown with each feed's error, and every feed shows its error.
