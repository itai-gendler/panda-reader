---
kind: validation
routes:
  desktop: Desktop
steps:
  - text: Enter a proxy URL with a user name and choose Save network settings
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
  - text: Refuse the settings with the reason and keep the previous ones
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::developer-settings
---

# Refuse invalid network settings

## Trigger

The proxy URL is not HTTP, HTTPS or SOCKS5h or contains credentials, or a CIDR range overlaps private or local networks.

## Outcome

The error is shown and the previous network settings stay in force.
