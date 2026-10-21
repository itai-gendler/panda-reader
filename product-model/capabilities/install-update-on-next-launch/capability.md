---
availability:
  - place: desktop
domain: updates
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/updater.rs#save_for_next_launch
  - kind: code
    role: implementation
    target: apps/panda-reader/src/updater.rs#apply_pending_on_launch
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/actions.rs#update_next_launch
---

# Install update on next launch

Choose **Next launch** to install a downloaded update the next time Panda Reader starts.
