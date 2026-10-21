---
availability:
  - place: desktop
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/developer.rs#render_developer_settings
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/actions.rs#save_article_network_settings
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/actions.rs#set_article_network_mode
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/articles/fetch.rs
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/diagnostics.rs
---

# Change developer settings

Set sync diagnostics and, under **Advanced settings**, the network used to fetch article pages for full text, on the **Developer** page.
