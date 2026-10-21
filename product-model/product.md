---
id: panda-reader
summary: >-
  A desktop RSS reader with a local library or a Miniflux or FreshRSS account, full-text extraction, article
  plugins and translation.
category: feed-reader
tags:
  - rss
  - feed-reader
  - desktop
  - miniflux
  - freshrss
authors:
  - name: yuhangch
    url: https://github.com/yuhangch
license: MIT
languages:
  - en
  - zh-Hans
  - zh-Hant
  - ja
  - fr
  - de
limitations:
  - >-
    Each subscription source keeps a separate library: Local, Miniflux and FreshRSS never share or merge
    feeds, articles or reading state.
  - >-
    In a Miniflux or FreshRSS library the provider manages subscriptions, folders and feed details; only title
    translation is set per feed in Panda Reader.
  - >-
    Read Later is kept on this computer only; read and starred state are the only reading state shared with a
    provider.
  - >-
    Article and title translation use an outside translation service the person configures with their own
    keys.
  - >-
    Article plugins only transform the article document through the app; they get no direct network, file,
    database or interface access.
references:
  - kind: doc
    role: intent
    target: README.md
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/index.md
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/providers.md
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/plugins/index.md
  - kind: doc
    role: context
    target: docs/src/content/docs/docs/development/architecture.md
  - kind: code
    role: implementation
    target: apps/panda-reader/src/app/mod.rs#run
  - kind: code
    role: implementation
    target: apps/panda-reader/src/app/preferences.rs#Language
---

# Panda Reader

Panda Reader is a desktop RSS reader for Windows, macOS and Linux. People read from a **Local** library that Panda Reader refreshes straight from the feeds, or connect a **Provider** (Miniflux or FreshRSS) whose subscriptions and read and starred state it synchronizes. Articles can be expanded to their full text from the original page, cleaned by article plugins, translated, and shared as a link or a quote image.

## Intent

"It's simply an RSS reader I wanted for myself, shaped around my own needs as I go." Surrounded by algorithmic feeds and an endless churn of updates, Panda Reader goes back to RSS, "for independent blogs: ordinary lives, without a big hook, told simply and honestly." Contributions are welcome, "and so are forks made to fit your own reading habits."
