---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Turn on Advanced settings and choose Explicit proxy
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Full-text fetch network
    contexts:
      desktop:
        place: desktop::developer-settings
  - text: Enter the proxy URL and choose Save network settings
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Proxy URL
    contexts:
      desktop:
        place: desktop::developer-settings
  - text: Check the proxy URL and save the network settings
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: changes
        facts:
          - Full-text fetch network
          - Proxy URL
          - Synthetic routing CIDR ranges
    contexts:
      desktop:
        place: desktop::developer-settings
---

# Fetch article pages through a proxy

## Trigger

Some publisher pages are only reachable through a proxy.

## Outcome

"Article network settings saved" is shown; only full-text downloads use the proxy, while RSS refresh and provider sync keep their usual route.
