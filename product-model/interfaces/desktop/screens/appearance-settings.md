---
entities:
  - entity: settings
    shows:
      - Theme
      - Interface size
      - Show panda icon
    collects:
      - Theme
      - Interface size
      - Show panda icon
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/appearance.rs#render_appearance_settings
---

# Appearance

The color theme, the interface size and the panda icon in the title bar.
