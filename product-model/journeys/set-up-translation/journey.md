---
actors:
  - reader
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/article_view/actions.rs#toggle_or_translate
---

# Set up translation from an article

## Goal

Translate an article the first time, configuring a translation provider when none is set up yet.

## Success criterion

The article is shown translated with the provider the Reader configured.
