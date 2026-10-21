---
entities:
  - entity: settings
    shows:
      - Detailed sync logging
      - Log retention
      - Full-text fetch network
      - Proxy URL
      - Synthetic routing CIDR ranges
    collects:
      - Detailed sync logging
      - Log retention
      - Full-text fetch network
      - Proxy URL
      - Synthetic routing CIDR ranges
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/developer.rs#render_developer_settings
---

# Developer

Sync diagnostics (detailed logging, log retention, log file path) and, under Advanced settings, the network used to fetch article pages for full text.
