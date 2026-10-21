---
singleton: true
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/app/preferences.rs#Preferences
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/settings/actions.rs#reset_settings
---

# Settings

The app-wide preferences kept on this computer, shared by every library.

## Information kept

- **Language** — the interface language: English, Simplified Chinese, Traditional Chinese, Japanese, French or German
- **Subscription source** — the library shown: Local subscriptions (the default), Miniflux or FreshRSS
- **Article font** — serif (the default) or sans-serif
- **Text size** — article text size, 14 to 30
- **Line height** — article line height, 1.2 to 2.2
- **Text width** — article text width, 520 to 1000
- **Paragraph spacing** — space between paragraphs, 0.5 to 2.5
- **Remember reading position** — whether each article reopens where the Reader left off; on by default
- **Collapse feed sidebar** — whether the feed column is collapsed
- **First-line indent** — whether article paragraphs start with two full-width spaces; on by default
- **Vim navigation** — whether single keys such as j, k, s, u and l navigate when not typing
- **Share template** — the text Share link copies; {title} and {url} are replaced and \n starts a new line
- **Auto extract full text** — whether opening an article without full text fetches its original page; on by default
- **Content extractor** — Readability (the default), Defuddle, Trafilatura or Heuristic
- **Hide images** — whether article images are hidden
- **Theme** — the color theme
- **Interface size** — the size of interface text and controls, 13 to 18 px
- **Show panda icon** — whether the panda is shown in the title bar
- **Article language** — the language articles and titles are translated into; Simplified Chinese by default
- **Automatically translate titles** — whether titles from feeds marked Translate titles are translated in the background
- **Only translate future titles** — whether automatic title translation skips articles already in the library; on by default
- **Display mode** — Immersive (each paragraph followed by its translation, the default) or Translation only
- **Check for updates automatically** — whether Panda Reader checks for releases and community plugin updates at start and every six hours; on by default
- **Detailed sync logging** — whether provider connection checks and per-page sync timings are written to the sync log
- **Log retention** — how long the sync log is kept: 7, 30 (the default) or 90 days
- **Full-text fetch network** — how article pages are fetched for full text: System proxy (the default), Direct connection, Explicit proxy or Trusted transparent proxy
- **Proxy URL** — the HTTP, HTTPS or SOCKS5h proxy used in Explicit proxy mode
- **Synthetic routing CIDR ranges** — address ranges a system or transparent proxy may route article pages through
- **Last refresh** — when the last refresh succeeded, shown in the status bar
