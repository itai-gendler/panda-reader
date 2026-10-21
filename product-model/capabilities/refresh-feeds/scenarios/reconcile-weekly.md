---
kind: alternate
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
  - text: The last full reconciliation is seven or more days old
    kind: condition
    entities:
      - entity: library
        effect: reads
        facts:
          - Last full reconciliation
    contexts:
      desktop:
        place: desktop::feeds
  - text: Read every entry from the provider, page by page, taking its read and starred state
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
        facts:
          - Starred
    contexts:
      desktop:
        place: desktop::feeds
  - text: Record the full reconciliation
    kind: product
    actor: reader
    entities:
      - entity: library
        effect: changes
        facts:
          - Last synchronized
          - Last full reconciliation
    contexts:
      desktop:
        place: desktop::feeds
---

# Reconcile the whole provider library weekly

## Trigger

A sync starts when the library has never been fully reconciled, or its last full reconciliation is seven or more days old.

## Outcome

Every entry is read from the provider rather than only recent changes, and the time of this full reconciliation is recorded.
