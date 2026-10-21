---
domain: plugins
references:
  - kind: code
    role: implementation
    target: crates/panda-plugins/src/manifest.rs#PluginManifest
  - kind: code
    role: implementation
    target: crates/panda-plugins/src/registry.rs#PluginSummary
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/plugins/manifest.md
---

# Article plugin

A site-specific plugin that repairs or cleans article HTML, or resolves a wrapped article link, for the pages its manifest matches. One rules plugin is built in; others are installed by the Reader.

## Information kept

- **ID** — the stable plugin identifier; IDs starting with builtin. are reserved
- **Name** — the plugin name
- **Version** — the plugin version, compared to offer community updates
- **Format** — rules (declarative TOML) or WASM
- **Stage** — prepare (repairs the downloaded page before extraction), cleanup (cleans feed content or extracted text) or resolve URL (finds the real article address before download)
- **Domains** — the host names the plugin matches; *.example.net matches the domain and its subdomains
- **Path prefixes** — URL paths the plugin is limited to, when given
- **Priority** — plugins run from the highest priority, then by ID
- **Host capabilities** — the document and network operations the plugin declares; undeclared operations fail
- **Network hosts** — the exact HTTPS hosts a URL resolver may ask the app to contact
- **Built-in** — whether the plugin ships with Panda Reader
- **Last error** — the error of its last failed run

## States

### Enabled

The plugin runs on matching pages.

### Disabled

The plugin is installed but does not run.
