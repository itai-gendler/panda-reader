---
appliesTo:
  - type: capability
    id: translate-article
  - type: capability
    id: translate-title
  - type: capability
    id: translate-titles
references:
  - kind: code
    role: implementation
    target: crates/panda-translate/src/lib.rs#is_configured
  - kind: code
    role: implementation
    target: crates/panda-translate/src/lib.rs#build
---

# Translation needs a configured provider

Translate, Translate title and automatic title translation work only when the selected translation provider has its keys (and, for language-model providers, an API URL and model). DeepL and LibreTranslate cannot be used yet.
