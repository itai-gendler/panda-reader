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
  - text: Find a newer release this installation cannot install itself
    kind: product
    actor: reader
    entities:
      - entity: app-update
        effect: creates
        to: Manual install required
        facts:
          - Version
          - Release page
          - Manual install reason
    contexts:
      desktop:
        place: desktop::about
---

# This installation needs a manual update

## Trigger

Panda Reader runs from the portable Windows ZIP or Linux tarball, or the release has no package for this installation.

## Outcome

About shows the new version with the reason and Open releases; nothing is downloaded.
