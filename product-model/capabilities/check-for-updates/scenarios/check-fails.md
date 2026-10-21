---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: Choose Check now
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::about
  - text: Report "Update check failed:" with the reason
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::about
---

# The check fails

## Trigger

The release information cannot be read or the package is invalid.

## Outcome

"Update check failed:" is shown with the reason.
