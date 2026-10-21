---
singleton: true
domain: translation
references:
  - kind: code
    role: implementation
    target: crates/panda-translate/src/lib.rs#TranslatorConfig
  - kind: code
    role: implementation
    target: crates/panda-translate/src/lib.rs#Provider
---

# Translation provider

The translation service Panda Reader sends article bodies and titles to, with the keys for each service. The keys of every service are kept, so switching back does not lose them.

## Information kept

- **Provider** — Azure Translator (the default), Volcengine, OpenAI-compatible, Anthropic or Gemini; DeepL and LibreTranslate are listed as coming soon
- **Azure API key** — the Azure Translator key
- **Azure region** — the Azure region, eastasia by default
- **Access Key ID** — the Volcengine access key ID
- **Secret Access Key** — the Volcengine secret access key
- **API URL** — the API address, kept separately for OpenAI-compatible, Anthropic and Gemini
- **API key** — the API key, kept separately for OpenAI-compatible, Anthropic and Gemini
- **Model** — the model, kept separately for OpenAI-compatible, Anthropic and Gemini
