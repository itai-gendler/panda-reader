---
availability:
  - place: desktop
domain: translation
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/translation.rs#render_translation_settings
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/actions.rs#persist_translator_settings
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/actions.rs#set_only_translate_future_titles
  - kind: code
    role: implementation
    target: crates/panda-translate/src/lib.rs#is_configured
---

# Change translation settings

Choose the article language, automatic title translation, the display mode and the translation provider with its keys on the **Translation** page.
