---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Start-up or six hours have passed and automatic checks are on
    kind: condition
    unattended: true
    entities:
      - entity: settings
        effect: reads
        facts:
          - Check for updates automatically
    contexts:
      desktop:
        place: desktop::about
  - text: Download and verify the newer release
    kind: product
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
  - text: Offer the verified update
    kind: product
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

# Check automatically

## Trigger

Panda Reader starts, or six hours pass, while Check for updates automatically is on.

## Outcome

A newer release is downloaded in the background and the status bar shows "Update ready".
