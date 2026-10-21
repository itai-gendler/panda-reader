---
appliesTo:
  - type: capability
    id: open-article
  - type: capability
    id: mark-as-read
  - type: capability
    id: mark-as-unread
  - type: capability
    id: star-article
  - type: capability
    id: remove-star
  - type: capability
    id: mark-all-as-read
  - type: capability
    id: save-for-later
  - type: capability
    id: remove-from-read-later
references:
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/articles.rs#mark
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/articles/mod.rs#queue_remote_mark_flush
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/tests.rs#miniflux_entries_keep_remote_state_and_local_later_flag
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/tests.rs#flush_keeps_a_newer_mark_queued_while_the_previous_value_is_in_flight
---

# Read and starred state reach the provider; Read Later stays here

In a provider library, every change to read or starred state (opening, marking, starring, Read all) is saved here at once and queued for the provider, then sent in the background and again at the start of each sync until the provider accepts it. A newer change to the same article replaces the queued one. Read Later is never sent and survives syncs.
