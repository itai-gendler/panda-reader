---
appliesTo:
  - type: capability
    id: translate-article
  - type: capability
    id: translate-title
  - type: capability
    id: translate-titles
  - type: entity
    id: translation-usage
references:
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/translation.rs#record_translation_usage_with_tokens
---

# Every translation request is counted in daily usage

Requests sent to a translation service, the characters sent and, for language-model services, the reported tokens are added to that day's usage for the service. For titles this includes the requests sent before a batch failed.
