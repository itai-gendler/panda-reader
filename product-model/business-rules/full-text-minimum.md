---
appliesTo:
  - type: capability
    id: extract-full-text
  - type: entity
    id: article
    facts:
      - Full text
references:
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/extraction.rs#prepare_extraction_from_page
---

# Full text must contain readable article text

An extraction that yields fewer than 80 characters of text is refused with "No extractable article content found on this page"; the article keeps what it had. When a prepare plugin selected the body but it is that short, the page is extracted again without the plugin's selection first.
