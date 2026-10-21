---
kind: primary
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
  - text: Find a newer stable release with a package for this installation
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::about
  - text: Download the package, showing progress
    kind: product
    actor: reader
    entities:
      - entity: app-update
        effect: creates
        to: Downloading
        facts:
          - Version
          - Download progress
          - Release page
    contexts:
      desktop:
        place: desktop::about
  - text: Verify the package's SHA-256 checksum and offer it
    kind: product
    actor: reader
    entities:
      - entity: app-update
        effect: changes
        from: Downloading
        to: Ready
        facts:
          - Download progress
    contexts:
      desktop:
        place: desktop::about
---

# Download a newer release

## Trigger

A newer stable version has been released.

## Outcome

The verified package is downloaded and About shows "Version {} is ready to install." with Update and restart, Next launch and Later.
