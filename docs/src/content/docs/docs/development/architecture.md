---
title: Architecture boundaries
description: Where application orchestration, persistent data, network work, and temporary UI state belong.
---

Panda Reader keeps the database as the authority for saved library state. The UI holds temporary state such as focus, scroll position, open panels, and request epochs. Service commands capture their workspace when queued, and database writes check that the target article belongs to that workspace.

## Main boundaries

| Area | Owns |
| --- | --- |
| `panda-core` | Shared article, feed, workspace, and translation types |
| `panda-store` | SQLite schema, queries, transactions, FTS, and persisted caches |
| `panda-providers` | Miniflux and FreshRSS protocol clients and provider settings |
| `panda-content` / `panda-plugins` | Host-owned article processing and plugin execution |
| `panda-translate` | Translation providers, segmentation, and translation settings |
| `apps/panda-reader/src/services` | Network orchestration, task routing, persistence calls, and cancellation |
| `apps/panda-reader/src/ui` | Temporary interaction state and rendering |

The store is split by concern under `crates/panda-store/src/store/`. HTTP retrieval for local RSS feeds and article pages lives in the application service layer. The store receives fetched input, prepares parsed feed data, and persists it. SQLite remains a single-writer database with WAL and independent read connections.

## Background work

`services/task_coordinator.rs` owns bounded general, synchronization, and title-translation lanes. Network-bound synchronization runs outside the general reader lane. Provider mark flushes are coalesced by workspace; translations share matching in-flight work; changing or closing an article cancels its article tasks. Store updates for extraction use a source revision check so a fetch started against old content cannot overwrite a newer article.

These lanes are an incremental coordinator, not a promise that every future task will need its own thread. New work should use the existing task identities and cancellation path where applicable, and should not block the GPUI thread.

## UI state

`ReaderWindow` composes focused state structs for the article list, reader, settings, sidebar, editor, and status bar. Persisted preferences are saved separately; articles and reading state are loaded from the store. Keep temporary UI state out of the database and treat service responses as versioned results before applying them to the current view.

Extract a subtree into its own GPUI entity when it needs an independent update lifecycle or measured rendering work shows that parent updates are costly. Simple views can remain state plus render methods; do not create an entity per control.

## Validation

- Automated formatting, workspace tests, Clippy, and native build checks run in GitHub Actions.
- Use the [GUI smoke test](./gui-smoke-test/) for desktop behavior CI cannot exercise, including IME composition, platform shortcuts, and X11/Wayland rendering.
