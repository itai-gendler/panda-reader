---
kind: alternate
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
  - text: Report "Panda Reader is up to date."
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::about
---

# Already up to date

## Trigger

The latest release is not newer than this version.

## Outcome

"Panda Reader is up to date." is shown.
