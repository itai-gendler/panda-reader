---
appliesTo:
  - type: capability
    id: check-for-updates
  - type: capability
    id: install-update-now
  - type: capability
    id: install-update-on-next-launch
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/updater.rs#check_and_download
  - kind: code
    role: implementation
    target: apps/panda-reader/src/updater.rs#launch_now
  - kind: doc
    role: intent
    target: README.md
---

# Updates install only from verified packages for this installation

Panda Reader downloads only the package made for how it is installed (Windows per-user setup, macOS app or Linux AppImage), verifies its SHA-256 checksum before offering it and again before installing it, and never installs an update the Reader has not chosen. Portable builds are updated by hand from the releases page.
