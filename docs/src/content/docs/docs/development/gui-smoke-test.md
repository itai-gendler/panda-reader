---
title: GUI smoke test
description: A short manual checklist for platform behavior that automated tests cannot verify.
---

Run this checklist on a release candidate or after changes to windowing, input, shortcuts, or rendering.
Automated CI covers formatting, Rust tests, static checks, and native compilation; it does not emulate a real desktop session.

## Common checks

- Launch the app with an existing library and with a fresh data directory.
- Open an article with long text, images, and translated content; resize the window and confirm the layout follows the available width.
- Search, switch feeds and workspaces, then open an article. Confirm the displayed article belongs to the selected workspace.
- Type into search, settings, and feed URL fields. Verify ordinary typing, paste, selection, and IME composition.
- Open and close settings, dialogs, image viewer, and command palette. Confirm focus returns to the expected control and reader shortcuts do not trigger behind modal UI.
- Start a sync, then continue opening and reading cached articles while it runs.

## Windows

- Use a Chinese IME to compose and commit text in every text field; verify candidate selection and paste.
- Check Ctrl-based shortcuts, window resize, minimize/restore, and high-DPI scaling.
- Start a slow provider sync and confirm cached articles remain readable.

## macOS

- Check Command-based shortcuts, especially search, settings, and article navigation.
- Verify focus restoration after closing the command palette and dialogs.
- Resize the window and inspect article images, text selection, and clipboard actions.

## Linux

- Run once under X11 and once under Wayland when both sessions are available.
- Confirm the top-level window has the Panda Reader identity and the full UI renders after startup.
- Check clipboard, resize, minimize/restore, and article image rendering.

Record the OS version, session type, display scaling, and exact reproduction steps when reporting a failure.
