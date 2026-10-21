---
domain: updates
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/updater.rs#UpdateStatus
  - kind: code
    role: implementation
    target: apps/panda-reader/src/updater.rs#ReadyUpdate
  - kind: code
    role: implementation
    target: apps/panda-reader/src/updater.rs#check_and_download
---

# App update

A newer stable Panda Reader release found by an update check, and the downloaded package waiting to be installed.

## Information kept

- **Version** — the release version
- **Download progress** — how much of the package has been downloaded
- **Release page** — the release page on GitHub
- **Manual install reason** — why this installation cannot update itself

## States

### Downloading

The package for this installation is being downloaded and verified.

### Ready

The verified package is downloaded: Update and restart, Next launch and Later are offered.

### Scheduled for next launch

The update will be installed the next time Panda Reader starts.

### Postponed

The Reader chose Later; the update is offered again at the next check.

### Manual install required

This installation cannot install the release itself; Open releases leads to the download.
