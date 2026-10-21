---
scope: >-
  The Panda Reader desktop app: libraries and provider sync, feeds, articles, full text, article plugins,
  translation, settings and app updates.
method: >-
  Static reading of the tracked UI, services, store, provider, content, plugin and translation crates at the
  pinned commit.
covered:
  - description: Desktop window, panes, dialogs and settings pages.
    paths:
      - apps/panda-reader/src/ui/
  - description: Application operations, background work, sync and article services.
    paths:
      - apps/panda-reader/src/services/
  - description: Startup, data paths and saved preferences.
    paths:
      - apps/panda-reader/src/app/
      - apps/panda-reader/src/main.rs
  - description: Release checks, downloads and the update installer.
    paths:
      - apps/panda-reader/src/updater.rs
      - apps/panda-reader/src/bin/
  - description: 'Library store: feeds, articles, search, OPML, extraction, translation caches and provider state.'
    paths:
      - crates/panda-store/src/
      - crates/panda-core/
  - description: Miniflux and FreshRSS clients and provider settings.
    paths:
      - crates/panda-providers/
      - crates/panda-miniflux/
  - description: Article cleaning, extraction and plugin execution, with the built-in plugin.
    paths:
      - crates/panda-content/
      - crates/panda-plugins/src/
      - plugins/builtin.wiki-cateat/
  - description: Translation services, batching and settings.
    paths:
      - crates/panda-translate/src/
exclusions:
  - description: Release packaging, CI workflows and published update manifests.
    paths:
      - .github/
      - about.hbs
      - about.toml
      - apps/panda-reader/build.rs
      - docs/public/updates/
      - .cargo/
  - description: Documentation site sources and plugin examples.
    paths:
      - docs/
  - description: >-
      Community plugin sources, catalog generator, plugin SDK, plugin fixture runner and agent authoring
      skill.
    paths:
      - plugins/community/
      - scripts/
      - crates/panda-plugin-sdk/
      - crates/panda-plugins/src/bin/
      - .agents/
  - description: Bundled fonts, icons and license notices.
    paths:
      - apps/panda-reader/assets/
      - THIRD_PARTY_LICENSES.txt
      - THIRD_PARTY_NOTICES.md
  - description: Tests, fixtures and the translation smoke example.
    paths:
      - apps/panda-reader/tests/
      - crates/panda-store/tests/
      - crates/panda-translate/examples/
unmapped:
  - description: Sidebar folder icons, folder order and collapsing, and the copy and open-in-browser menu items.
    paths:
      - apps/panda-reader/src/ui/sidebar/actions.rs
      - apps/panda-reader/src/ui/context_menus.rs
  - description: Command palette, keyboard shortcut catalog and copy-title or copy-link commands.
    paths:
      - apps/panda-reader/src/ui/commands/
  - description: Upgrade migration of earlier configuration and database layouts.
    paths:
      - apps/panda-reader/src/app/config.rs
      - crates/panda-store/src/store/schema.rs
  - description: Feed icon downloads and caching.
    paths:
      - apps/panda-reader/src/services/favicon.rs
  - description: Sync log writing, rotation and retention cleanup.
    paths:
      - apps/panda-reader/src/services/diagnostics.rs
  - description: Image viewer and window chrome.
    paths:
      - apps/panda-reader/src/ui/window.rs
limitations: []
---

# Coverage
