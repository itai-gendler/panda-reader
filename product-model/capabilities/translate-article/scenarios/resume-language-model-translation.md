---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: Choose Translate on the open article
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::article
  - text: Send the paragraphs to the language model in batches and show each finished batch
    kind: product
    actor: reader
    entities:
      - entity: translation-provider
        effect: reads
        facts:
          - Provider
          - Model
      - entity: article
        effect: reads
        facts:
          - Feed content
          - Full text
    contexts:
      desktop:
        place: desktop::article
  - text: Report that the batch timed out and that completed paragraphs are saved
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::article
  - text: Choose Translate again
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::article
  - text: Translate only the missing paragraphs and save the whole translation
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
        facts:
          - Translation
          - Translated title
    contexts:
      desktop:
        place: desktop::article
---

# Resume a language-model translation

## Trigger

An OpenAI-compatible, Anthropic or Gemini translation stops part-way, for example after a 60-second batch timeout or a rate limit.

## Outcome

Paragraphs appeared as their batches finished and were saved one by one; the error says completed paragraphs are saved, and translating again sends only the paragraphs still missing.
