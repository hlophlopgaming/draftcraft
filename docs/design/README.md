# Redesign mockups (v0.6)

Three directions for the new main window, for the owner to choose from in the issue
"Redesign: choose a direction". Nothing here is app code: the chosen variant becomes the design
tokens in `crates/ui-egui/src/theme.rs` and the window layout.

| Variant | Light | Dark |
|---|---|---|
| **A · Paper**: calm, warm, one slim top bar, icon rail + collapsible left sidebar, right context panel | ![](mockup-a-light.png) | ![](mockup-a-dark.png) |
| **B · Studio**: every view is a dockable panel with tabs and splitters, layout presets, cream paper even in dark | ![](mockup-b-light.png) | ![](mockup-b-dark.png) |
| **C · Focus**: almost no chrome, floating tool rail, scene minimap, command palette as the main way in | ![](mockup-c-light.png) | ![](mockup-c-dark.png) |

All three share: the element of the current line in the left margin of the page, a command
palette (every command by id), the sample script "Night Signal", Courier Badi on the page.
The interface font in the mockups (Inter) is a stand-in until v0.8 chooses the bundled UI font.

Edit `redesign-mockups.html` (open it with `?v=a|b|c&theme=light|dark`) and re-render with
`node docs/design/render.mjs` (Playwright + Chromium).
