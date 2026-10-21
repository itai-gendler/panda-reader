---
availability:
  - place: desktop
domain: updates
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/updater.rs#start_check
  - kind: code
    role: implementation
    target: apps/panda-reader/src/updater.rs#check_and_download
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/actions.rs#check_for_updates
  - kind: doc
    role: intent
    target: README.md
---

# Check for updates

Look for a newer stable release with **Check now**, or automatically at start-up and every six hours, and download it in the background when this installation can update itself.
