---
availability:
  - place: desktop
domain: updates
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/updater.rs#launch_now
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/actions.rs#update_now
---

# Update and restart

Install a downloaded update now: Panda Reader closes, the updater installs the package and the new version starts.
