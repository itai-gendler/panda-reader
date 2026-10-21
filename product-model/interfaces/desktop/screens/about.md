---
entities:
  - entity: app-update
    shows:
      - Version
      - Download progress
      - Release page
      - Manual install reason
  - entity: settings
    shows:
      - Check for updates automatically
    collects:
      - Check for updates automatically
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/about.rs#render_about_settings
  - kind: code
    role: implementation
    target: apps/panda-reader/src/updater.rs#UpdateStatus
---

# About

The app version, update checks and installation choices, community plugin update count, and acknowledgements.
