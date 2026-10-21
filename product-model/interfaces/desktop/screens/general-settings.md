---
entities:
  - entity: settings
    shows:
      - Language
      - Subscription source
    collects:
      - Language
      - Subscription source
  - entity: miniflux-connection
    shows:
      - Server URL
      - API token
    collects:
      - Server URL
      - API token
  - entity: freshrss-connection
    shows:
      - Server URL
      - Username
      - API password
    collects:
      - Server URL
      - Username
      - API password
  - entity: feed
    shows:
      - Title
      - Unread count
    collects:
      - Feed URL
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/general.rs#render_general_settings
---

# General

The interface language, the subscription source and its provider connection, feed management (add a feed, sync, OPML import and export), the searchable list of added feeds, and reset settings.
