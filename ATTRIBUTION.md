# Attribution

Every non-code asset in this repository, with its author, source and licence. `cargo xtask assets`
(part of `cargo xtask ci`) fails if a font, image or sample script is missing from this list.

DraftCraft contains **no** Final Draft icons, images, templates, sample scripts, fonts or wording
(beyond feature names used for comparison). Interface icons are drawn in code (`crates/ui-egui/src/icons.rs`). The app icon is a public-domain
(CC0) stag-beetle engraving from Openclipart, listed under Images below.

## Fonts

| File | Author | Source | Licence |
|---|---|---|---|
| `crates/fonts/fonts/CourierBadi-Regular.ttf` | Alan Dague-Greene (Courier Prime), Dmitry Novikov (Cyrillic), LaPingvino and the Courier Badi contributors | https://github.com/LaPingvino/courier-badi/releases/tag/v1.010 (`fonts/ttf`, not the contrast variant) | SIL Open Font License 1.1 (`crates/fonts/fonts/OFL.txt`) |
| `crates/fonts/fonts/CourierBadi-Bold.ttf` | same | same | SIL OFL 1.1 |
| `crates/fonts/fonts/CourierBadi-Italic.ttf` | same | same | SIL OFL 1.1 |
| `crates/fonts/fonts/CourierBadi-BoldItalic.ttf` | same | same | SIL OFL 1.1 |

## Word lists

| File | Author | Source | Licence |
|---|---|---|---|
| `crates/spell/words/en.txt` | Kevin Atkinson (English Speller Database / SCOWL), from 12dicts and ENABLE2K (public domain) | https://app.aspell.net/create (size 60, US + British -ise) | ESDB licence: free use, copy, modification and distribution with the copyright notice (`crates/spell/words/LICENSE-SCOWL.txt`) |
| `crates/spell/words/common.txt` | same | same (size 35) | same |

## Sample script

| File | Author | Licence |
|---|---|---|
| `crates/engine/src/sample.fountain` ("Night Signal") | The DraftCraft contributors, written for this project | CC0 1.0 (public domain dedication) |

## Images

| File | Author | Source | Licence |
|---|---|---|---|
| `docs/images/hero.png` | The DraftCraft contributors | Screenshot of DraftCraft rendered by `ui_shot` | MIT OR Apache-2.0 |
| `docs/images/cards.png` | The DraftCraft contributors | Screenshot of DraftCraft rendered by `ui_shot` | MIT OR Apache-2.0 |
| `docs/images/reports.png` | The DraftCraft contributors | Screenshot of DraftCraft rendered by `ui_shot` | MIT OR Apache-2.0 |
| `docs/demo/pr33-tour.mp4` | The DraftCraft contributors | Recording of DraftCraft by `ui_demo` (docs/demo/record.sh) | MIT OR Apache-2.0 |
| `docs/images/dark-smarttype.png` | The DraftCraft contributors | Screenshot of DraftCraft rendered by `ui_shot` | MIT OR Apache-2.0 |
| `docs/images/russian.png` | The DraftCraft contributors | Screenshot of DraftCraft rendered by `ui_shot` from `docs/demo/russian-tour.jsonl` | MIT OR Apache-2.0 |
| `docs/images/icon.svg` | Openclipart contributor (engraving); the DraftCraft contributors (tile) | Engraving "Beetle (Lucanus cinnamomeus)" from Openclipart (https://openclipart.org/detail/264956/beetle-lucanus-cinnamomeus), unchanged, on a sand tile with a paper backing traced from it by the DraftCraft contributors | CC0 1.0 (public domain dedication) |
| `docs/images/icon-512.png` | same | Rendered from `docs/images/icon.svg` | CC0 1.0 |
| `apps/draftcraft/assets/icon-256.png` | same | Rendered from `docs/images/icon.svg`; the window icon | CC0 1.0 |
| `docs/design/mockup-a-light.png` | The DraftCraft contributors | Redesign mockup rendered from `docs/design/redesign-mockups.html` by `docs/design/render.mjs` | MIT OR Apache-2.0 |
| `docs/design/mockup-a-dark.png` | The DraftCraft contributors | Redesign mockup rendered from `docs/design/redesign-mockups.html` by `docs/design/render.mjs` | MIT OR Apache-2.0 |
| `docs/design/mockup-b-light.png` | The DraftCraft contributors | Redesign mockup rendered from `docs/design/redesign-mockups.html` by `docs/design/render.mjs` | MIT OR Apache-2.0 |
| `docs/design/mockup-b-dark.png` | The DraftCraft contributors | Redesign mockup rendered from `docs/design/redesign-mockups.html` by `docs/design/render.mjs` | MIT OR Apache-2.0 |
| `docs/design/mockup-c-light.png` | The DraftCraft contributors | Redesign mockup rendered from `docs/design/redesign-mockups.html` by `docs/design/render.mjs` | MIT OR Apache-2.0 |
| `docs/design/mockup-c-dark.png` | The DraftCraft contributors | Redesign mockup rendered from `docs/design/redesign-mockups.html` by `docs/design/render.mjs` | MIT OR Apache-2.0 |
| `docs/design/focus/av-dark.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
| `docs/design/focus/av-light.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
| `docs/design/focus/cards-dark.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
| `docs/design/focus/cards-light.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
| `docs/design/focus/navigator-dark.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
| `docs/design/focus/navigator-light.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
| `docs/design/focus/overview-dark.png` | The DraftCraft contributors | Overview of the Focus mockups, composed from the renders of `docs/design/focus.html` (DejaVu Sans captions, Bitstream Vera licence) | MIT OR Apache-2.0 |
| `docs/design/focus/overview-light.png` | The DraftCraft contributors | Overview of the Focus mockups, composed from the renders of `docs/design/focus.html` (DejaVu Sans captions, Bitstream Vera licence) | MIT OR Apache-2.0 |
| `docs/design/focus/panels-dark.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
| `docs/design/focus/panels-light.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
| `docs/design/focus/rtl-dark.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
| `docs/design/focus/rtl-light.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
| `docs/design/focus/stats-dark.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
| `docs/design/focus/stats-light.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
| `docs/design/focus/typewriter-dark.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
| `docs/design/focus/typewriter-light.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
| `docs/design/focus/welcome-dark.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
| `docs/design/focus/welcome-light.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
| `docs/design/focus/write-dark.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
| `docs/design/focus/write-light.png` | The DraftCraft contributors | Focus redesign mockup rendered from `docs/design/focus.html` by `docs/design/render-focus.mjs`; sample script "Night Signal" (CC0), app icon (CC0), Hebrew screen in Liberation Mono (SIL OFL 1.1) | MIT OR Apache-2.0 |
