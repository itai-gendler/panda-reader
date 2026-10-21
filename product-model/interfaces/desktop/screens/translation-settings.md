---
entities:
  - entity: settings
    shows:
      - Article language
      - Automatically translate titles
      - Only translate future titles
      - Display mode
    collects:
      - Article language
      - Automatically translate titles
      - Only translate future titles
  - entity: translation-provider
    shows:
      - Provider
      - Azure API key
      - Azure region
      - Access Key ID
      - Secret Access Key
      - API URL
      - API key
      - Model
    collects:
      - Provider
      - Azure API key
      - Azure region
      - Access Key ID
      - Secret Access Key
      - API URL
      - API key
      - Model
  - entity: translation-usage
    shows:
      - Day
      - Provider
      - Requests
      - Characters
      - Input tokens
      - Output tokens
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/translation.rs#render_translation_settings
---

# Translation

The article language, automatic title translation, the display mode, the translation provider with its keys, and the daily translation usage.
