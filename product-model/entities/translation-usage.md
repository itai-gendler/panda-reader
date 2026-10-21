---
domain: translation
references:
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/translation.rs#record_translation_usage_with_tokens
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/translation.rs#translation_usage
---

# Translation usage

The daily count of translation requests sent to each translation service, shown under Translation usage (daily).

## Information kept

- **Day** — the local date of the requests
- **Provider** — the translation service
- **Requests** — how many requests were sent
- **Characters** — how many characters were sent
- **Input tokens** — tokens the service reports reading, for language-model services
- **Output tokens** — tokens the service reports writing, for language-model services
