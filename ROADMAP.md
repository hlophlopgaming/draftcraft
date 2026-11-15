# DraftCraft roadmap

**Stage:** v0.5.1 (released 2026-11-14), the first release as DraftCraft.
**Headline numbers:** 63 of 68 Final Draft 13 catalog features implemented
([docs/parity-checklist.md](docs/parity-checklist.md)); 194 engine commands + 53 window commands;
291 tests; English & Russian UI with Cyrillic screenplay support (Courier Badi); native builds for
Windows, macOS and Linux on x86_64 and ARM64, plus the browser; Rust 1.100.

DraftCraft is a fork of [ScriptCraft](https://github.com/NoJuanNobody/scriptcraft). Issue numbers
in the history below (#1–#54) are ScriptCraft's issues; new work gets issues in this repository.

## Direction

A screenwriting app for writers in **any language**, with **one set of page rules** (layout,
pagination, (MORE)/(CONT'D)) and each language's own words for them; **two kinds of script**
(the screenplay and the two-column audio/visual script); a **clean, modern interface**; and
open files that other tools read.

**A writing app, not a project manager.** You sit down and write the script; DraftCraft keeps
what the script itself needs (structure, cards, notes, reports). Research, world-building, mind
maps and relationship graphs belong in tools made for them (Obsidian and the like), not here.

## Plan

Order and why: the redesign comes first, so every later feature is built once, in the new
look. Fonts come before languages, because Arabic, Hebrew and CJK need fonts that have them.
Right-to-left is easier once the redesign has settled where panels and menus live.

### v0.6: redesign

The interface looks dated and crowded. A design pass before new features land:

- [ ] Design system: colours, type scale, spacing, icons, light and dark themes, all as tokens
      in `crates/ui-egui/src/theme.rs` (which already holds the colours), checked for contrast
- [ ] New window layout: a slimmer toolbar, a left sidebar (navigator, cards, outline, beats,
      notes) that collapses, a right panel for context (scene info, notes, revisions)
- [ ] **Movable panels**, like the older Final Draft releases but freer: every view (page,
      navigator, index cards, outline, beat board, story map, reports, notes, scene properties,
      revisions) is a panel the writer can drag, dock to any side, stack as tabs, split and
      resize. Built on a docking library for egui 0.37 (egui_tiles 0.18 or egui_stack 0.25,
      chosen after a spike); today's split screen (`split.rs`) becomes part of it
- [ ] Layouts saved in settings, with presets ("Writing", "Structure", "Production") and
      "Reset layout"; adaptive on narrow windows (panels collapse into tabs) and in the browser
- [ ] Right-to-left layouts: the presets mirror for Arabic and Hebrew (navigator on the right),
      and panel contents follow the interface direction (with v0.9)
- [ ] Panels torn out into separate OS windows on the desktop (egui viewports), e.g. index
      cards on a second monitor; not in the browser
- [ ] Calmer page view: paper on a neutral desk, quieter page breaks and margins, the element
      of the current line shown in the margin
- [ ] Redesigned dialogs: element settings, title page, page setup, find and replace,
      collaboration, print
- [ ] A command palette (every command by name; they already all have ids)
- [ ] Welcome screen: new script, templates, recent files, the guide
- [ ] Interface text size and theme choice in settings; keep it usable at 100–200 % scale
- [ ] Translate the remaining English-only menu items (Split Screen, Read Aloud, Track Changes,
      Bookmarks, New from Template, Add Structure Point, Collaborate…)
- [ ] New screenshots for README, the website and the guide

### v0.7: two-column (audio/visual) script

For documentaries, commercials, video essays, news and training films: picture on the left,
sound on the right, lined up row by row.

- [ ] A second document kind, alongside the screenplay: rows of **Video** | **Audio** cells,
      with optional scene number and duration per row
- [ ] Typing in the DraftCraft way: Tab moves between the columns, Enter adds a row; audio
      cells hold narration (VO), dialogue, music and sound effects
- [ ] Timing: estimated read time of the narration per row and in total
- [ ] Two-column PDF and printing; its own file format, and Final Draft's two-column format if
      public samples allow
- [ ] Convert a screenplay to two columns and back (action → Video, dialogue → Audio)
- [ ] Template: "Two-column script"; MCP/CLI commands like every other feature

### v0.8: fonts

- [ ] Bundled screenplay fonts with open licences, each taken from its upstream source and
      checked for licence and coverage: Courier Prime (OFL), Courier Badi (already), Cousine and
      Liberation Mono (Courier New metrics, with Hebrew), a monospaced Arabic font (candidate:
      Kawkab Mono), Noto Sans Mono
- [ ] Fonts for scripts beyond Latin and Cyrillic, from upstream (OFL): Noto families, Mukta Malar
      (Tamil), Mallanna (Telugu). Interface fonts for the redesign: Noto Sans, Roboto or
      Montserrat (OFL). Arial, Times New Roman and Courier New are commercial and are never
      bundled; writers who have them pick them as installed fonts
- [ ] CJK (Noto Sans Mono CJK): downloaded on request, not bundled (≈16 MB each)
- [ ] Choose any installed font or a `.ttf`/`.otf` file; the choice is saved with the script
      and the font is embedded in the PDF
- [ ] Per-language fallback fonts, so a missing letter comes from another font instead of a box
- [ ] Watermarks on printed pages and PDFs (e.g. the recipient's name)

### v0.9: languages and right-to-left

The same page rules for everyone, in each writer's language.

- [ ] **Script language** (per script, separate from the interface language), which sets:
      scene heading prefixes (INT./EXT., ИНТ./НАТ., INNEN/AUSSEN, פנים/חוץ, داخلي/خارجي…),
      times of day, transitions, (MORE)/(CONT'D) and their translations, SmartType lists
- [ ] Arabic and Hebrew written right to left: bidirectional text (`unicode-bidi`) in the page
      view, caret and selection, PDF and print; mixed RTL/LTR lines (numbers, Latin names)
- [ ] Mirrored interface for right-to-left interface languages
- [ ] **Regional formatting standards**: a script follows a named standard, chosen when it is
      created and changeable later, e.g. US/Hollywood, UK/BBC, Russian, French, German. A
      standard is a bundle of what DraftCraft already has per script (element settings: indents,
      spacing, capitals, alignment; page setup: paper, margins, lines per page; header/footer)
      plus the language words above. Shipped as data files, so a new standard is a new file;
      a writer can save their own
- [ ] Spelling check per language (open dictionaries), thesaurus
- [ ] More interface languages: contributors add a catalogue file
      (`crates/ui-egui/locales/<code>.json`, see README); no translation platform needed

### v0.10: writing comfort

- [ ] Focus / typewriter mode
- [ ] Format assistant: a cue with no dialogue, an empty scene, two parentheticals in a row…
      (engine commands `assist.check`, `assist.next`, `assist.previous` done; a panel in the
      window still to come)
- [ ] Rename a character everywhere (engine command `character.rename` done; a dialog in the
      window still to come); highlight one character's lines; read one character's lines on
      their own
- [ ] **Writer's statistics**, a panel with charts (drawn in the app, light and dark themes):
      - the script: pages, scenes, words, estimated screen time; dialogue vs action; each
        character's share of the dialogue and scenes; scene lengths along the script; INT/EXT,
        day/night and locations; how the script grew draft by draft
      - this session: words and pages written, time spent, pace, what changed
      - over time: words a day as a calendar heat map and a bar chart, streaks, best days
      - kept on the writer's own computer only (data folder), never sent anywhere; export as CSV
        and as a picture
- [ ] Writing goals (pages or words a day, a deadline for a draft) shown against the
      statistics; goals and deadlines exported to any calendar as `.ics`, later synced over CalDAV
- [ ] Screen-time estimate per scene and for the script

### v0.11: story tools

Kept small, in line with "a writing app, not a project manager".

- [ ] **Character and location cards** (a light bible that fills itself): every character and
      location the script uses appears on its own, in the Navigator's Characters and Locations
      tabs; nothing has to be filled in
      - the writer may add: a one- or two-line description, a picture, a colour, other names
        for the same character or place (МАМА = НЕЛЛ, КУХНЯ = КУХНЯ НЕЛЛ) and free notes; the
        read-aloud voice moves here
      - counted by DraftCraft: first appearance, scenes, lines and share of the dialogue (or
        scenes, pages and day/night for a location), each scene a link to it
      - used while writing: hover a cue to see the description; one character's lines in their
        colour; a cast list ("Действующие лица") page from the descriptions; statistics that
        count a character once under all their names
      - kept in the script file itself; no family trees, relationship graphs, world-building or
        long biographies
- [ ] Story days and a timeline: which day of the story each scene is in, scenes laid out along
      the story's time, with a report
- [ ] Snapshots inside the file ("draft 1", "after the table read") to look at or go back to

### v0.12: formats and storage

- [ ] **OpenDocument (.odt)**: open and save, for LibreOffice and other open office suites
- [ ] Export to Word (.docx), RTF, plain text and HTML (import from Word already works)
- [ ] Import from KIT Scenarist, from its publicly documented or observed file structure only
      (KIT Scenarist and Story Architect are GPL: their code is never read)
- [ ] Open and save Fade In, Highland, Trelby and WriterDuet files where their formats are public
- [ ] Your own templates (save a script as a template); Final Draft templates (.fdxt)
- [ ] Subtitles (SRT) from the two-column script and from bilingual scripts
- [ ] Final Draft compatibility left over from v0.5 (#42, #44, #32): needs real Final Draft 13 files
- [ ] **WebDAV**: open and save scripts on Nextcloud, ownCloud, Yandex Disk, Synology/QNAP or any
      WebDAV server (address + app password). Saves check the file's version (ETag) and merge
      someone else's changes with the collaboration three-way merge instead of overwriting; a
      WebDAV lock marks the script "being edited by …" for others. (Browsers need the server to
      allow cross-origin requests; the desktop app does not.)
- [ ] Synced folders (Google Drive, Dropbox, OneDrive desktop apps): when the open file changes
      on disk, merge the changes instead of overwriting them

### v0.13: share by link

Collaboration over the internet without an account: the app stays the host, as in today's
local-network sessions (docs/collaboration.md); a small relay only forwards bytes.

- [ ] **Share** gives a link (`…/s/<room>#<key>`); **Stop** ends it. Anyone with the link joins
      from the app or from the browser (the web app), as viewer, commenter or editor
- [ ] Relay server: a small Rust service (or a Cloudflare Worker) that pairs the host with its
      guests and stores nothing; anyone can run their own
- [ ] End-to-end encryption: the key is in the link's `#` part, which never reaches the relay
- [ ] WebSocket transport for the collaboration protocol, so browsers can join
- [ ] Comment threads with replies and resolve, on top of script notes

### 1.0: ready for everyone

- [ ] Windows installer (MSI) with the exe icon and file associations (.fdx, .fountain)
- [ ] Signed and notarized macOS app; file associations
- [ ] Check for updates from inside the app
- [ ] Settings kept in the web app between visits
- [ ] Custom keyboard shortcuts
- [ ] Accessibility: screen readers (egui's AccessKit), keyboard-only use
- [ ] Every release build launched and checked on real Windows, macOS and Linux machines
- [ ] A new trailer and explainer for 1.0 (the old ones were removed until then)

### After 1.0

- [ ] More kinds of writing, each a document kind of its own like the two-column script:
      audio play / podcast / radio drama, comic book script (pages, panels, balloons), prose
- [ ] Episode outline for series (поэпизодник): seasons and episodes, the logline and beats of
      each episode, linked to the episode scripts
- [ ] Bilingual scripts: a translation kept next to each line, printable either way
- [ ] Table-read mode with roles assigned to people
- [ ] Production: sides for a shooting day, a simple stripboard and cast lists per day; the
      shooting schedule in the crew's calendars over CalDAV
- [ ] A DraftCraft cloud with accounts, where scripts live on a server: only if the project
      grows enough to pay for running it, building on share-by-link
- [ ] Tablets (iPad, Android) with a touch interface
- [ ] A GitHub organization for DraftCraft, like storytold for ArtCraft (optional)

### Not planned

Considered and left out, to keep DraftCraft a focused writing app:

- A project holding many documents, research, images and story worlds (as Story Architect does)
- Mind maps and character relationship graphs
- A translation platform (a catalogue file per language is enough)
- Fonts copied from other apps' repositories: fonts come only from their upstream sources with
  their licence

## Done

### v0.1: the editor (done)

- [x] Screenplay model, undo/redo, element flow, SmartType, dual dialogue
- [x] Industry pagination with (MORE)/(CONT'D), title page, scene numbers
- [x] Fountain and FDX round trips; PDF export (krilla, embedded Courier Badi)
- [x] Navigator, index cards with summaries and reordering, reports
- [x] egui window: menu bar, element picker, page view, status bar, dark mode, zoom
- [x] CLI, MCP server, keyed control channel, headless `ui_shot`

### v0.2: writing comfort (done)

- [x] [#1](https://github.com/NoJuanNobody/scriptcraft/issues/1) Inline bold, italic and underline
- [x] [#2](https://github.com/NoJuanNobody/scriptcraft/issues/2) Spelling check (offline)
- [x] [#3](https://github.com/NoJuanNobody/scriptcraft/issues/3) Element settings editor (indents, spacing, capitals, Enter/Tab targets)
- [x] [#4](https://github.com/NoJuanNobody/scriptcraft/issues/4) Decide: Tab in an empty Action → Character or Transition?
- [x] [#5](https://github.com/NoJuanNobody/scriptcraft/issues/5) Element picker on Enter in an empty paragraph
- [x] [#6](https://github.com/NoJuanNobody/scriptcraft/issues/6) Copy and paste that keeps element types
- [x] [#7](https://github.com/NoJuanNobody/scriptcraft/issues/7) Drag to reorder index cards; card colours and tags
- [x] [#8](https://github.com/NoJuanNobody/scriptcraft/issues/8) Autosave, backups and recent files
- [x] [#9](https://github.com/NoJuanNobody/scriptcraft/issues/9) Caret blink, wheel/pinch zoom, page-width zoom and a non-paginated speed view
- [x] [#10](https://github.com/NoJuanNobody/scriptcraft/issues/10) Verify the unsaved-changes prompt on close in the native window
- [x] [#11](https://github.com/NoJuanNobody/scriptcraft/issues/11) Remove the control key file when the app is terminated
- [x] [#12](https://github.com/NoJuanNobody/scriptcraft/issues/12) Web build (wasm)

### v0.3: production (done)

- [x] [#13](https://github.com/NoJuanNobody/scriptcraft/issues/13) Revision mode with coloured revision pages
- [x] [#14](https://github.com/NoJuanNobody/scriptcraft/issues/14) Locked pages, A/B pages, omitted scenes and locked scene numbers
- [x] [#15](https://github.com/NoJuanNobody/scriptcraft/issues/15) Headers and footers editor; A4 paper
- [x] [#16](https://github.com/NoJuanNobody/scriptcraft/issues/16) Script notes
- [x] [#17](https://github.com/NoJuanNobody/scriptcraft/issues/17) Alternate dialogue lines
- [x] [#18](https://github.com/NoJuanNobody/scriptcraft/issues/18) Production tagging and breakdown reports
- [x] [#19](https://github.com/NoJuanNobody/scriptcraft/issues/19) Print dialog
- [x] [#20](https://github.com/NoJuanNobody/scriptcraft/issues/20) Split dual dialogue across a page break

Follow-ups found while testing v0.3 (done in v0.5):

- [x] [#41](https://github.com/NoJuanNobody/scriptcraft/issues/41) Reports show counted page numbers instead of printed labels (12A) when pages are locked
- [x] [#43](https://github.com/NoJuanNobody/scriptcraft/issues/43) Print directly to a printer (printer list, copies), browser print, temp file cleanup
- [x] [#45](https://github.com/NoJuanNobody/scriptcraft/issues/45) Dual dialogue: a long cue plus (CONT'D) runs past the right column and margin after a page split

### v0.4: structure and collaboration (done)

- [x] [#21](https://github.com/NoJuanNobody/scriptcraft/issues/21) Outline view with structure points
- [x] [#22](https://github.com/NoJuanNobody/scriptcraft/issues/22) Beat board
- [x] [#23](https://github.com/NoJuanNobody/scriptcraft/issues/23) Story map
- [x] [#24](https://github.com/NoJuanNobody/scriptcraft/issues/24) Track changes and comparing two drafts
- [x] [#25](https://github.com/NoJuanNobody/scriptcraft/issues/25) Real-time collaboration
- [x] [#26](https://github.com/NoJuanNobody/scriptcraft/issues/26) Templates: TV single-camera, multi-camera, stage play
- [x] [#27](https://github.com/NoJuanNobody/scriptcraft/issues/27) Read aloud with cast voices, and dictation (speech to script)
- [x] [#28](https://github.com/NoJuanNobody/scriptcraft/issues/28) Bookmarks and split screen

### v0.5: fidelity and polish

File-format fidelity:

- [x] [#29](https://github.com/NoJuanNobody/scriptcraft/issues/29) Keep Shot and General elements in Fountain
- [x] [#30](https://github.com/NoJuanNobody/scriptcraft/issues/30) Keep all title page fields in FDX; support free-form title pages
- [x] [#31](https://github.com/NoJuanNobody/scriptcraft/issues/31) Import from Word, Celtx and PDF
- [ ] [#42](https://github.com/NoJuanNobody/scriptcraft/issues/42) Final Draft compatibility: map revisions, locks, omitted scenes, tags, alternates and page setup to Final Draft's own FDX — Final Draft's form written wherever public files confirm it ([docs/fdx-format.md](docs/fdx-format.md)); tags, omitted scenes, alternates, locks and tracked changes wait on #44
- [ ] [#44](https://github.com/NoJuanNobody/scriptcraft/issues/44) Get sample .fdx files from Final Draft 13 (tags, omitted scene, alternate line, locked pages)

Verification:

- [ ] [#32](https://github.com/NoJuanNobody/scriptcraft/issues/32) Check the typing flow against real Final Draft 13

## Progress log

- 2026-10-12 — v0.1: model, Fountain/FDX/PDF, pagination, engine with 73 commands, MCP, CLI,
  egui window with page view, navigator, index cards, reports, dark mode; README screenshots.
- 2026-10-12 — published at github.com/hlophlopgaming/draftcraft; 32 issues filed for the known gaps.
- 2026-10-12 — settings through craft-settings (storytold/craft-libs#22), the Craft suite's
  shared config.toml store and folder rules; recovery, backups and dictionary in the data folder (#35).
- 2026-10-12 — v0.2 writing comfort (#1–#12): inline styles, spelling, element settings, element
  list on Enter, structured copy/paste, card colours/tags and drag reorder, autosave/recovery/
  backups/recent files, caret blink, zoom and speed view, verified quit prompt, signal-safe
  control key, browser build.
- 2026-10-13 — CI on GitHub Actions for Linux, macOS and Windows, plus a weekly run against the
  newest Rust and dependencies (#34).
- 2026-10-13 — v0.3 production (#13–#20): revision mode with coloured sets, margin asterisks and
  revised-page headers; locked pages with A/B pages, omitted scenes and locked scene numbers;
  header/footer editor and A4 paper; script notes; alternate dialogue lines; production tagging
  with a breakdown report and CSV; File › Print; dual dialogue split across page breaks.
- 2026-10-14 — v0.4 structure and collaboration (#21–#28): acts, sequences and sections with an
  Outline view and a story map; beat board with beats linked to scenes; bookmarks and split screen;
  track changes per author with accept/reject, and Compare Drafts (also `draftcraft-cli compare`);
  LAN real-time collaboration with presence (docs/collaboration.md); TV single-camera,
  multi-camera and stage-play templates; read aloud with a voice per character and dictation.
- 2026-10-14 — v0.5 fidelity and polish (#29–#31, #41–#43, #45): Shot and General kept in
  Fountain; free-form and complete FDX title pages; Final Draft's own FDX forms for revisions,
  page setup, notes, voices and options (docs/fdx-format.md), which also fixed real Final Draft
  files opening fully revised; import from Word, Celtx and PDF; reports with printed page labels
  on locked pages; continued dual-dialogue cues kept in their column; printing to a printer.
- 2026-10-26 — v0.5.0 release (#48, #49, #50, #51, #53, #54): Russian localization and Cyrillic
  screenplay support with embedded Courier Badi v1.010; interactive architecture & workflow
  explainer and live browser app on GitHub Pages; multi-platform release builds for Windows (x64),
  macOS (Universal `.app` + CLI), Linux (`x86_64` tarball and `.deb`), and Web (WASM).
- 2026-11-14 — v0.5.1, the first release as DraftCraft (fork of ScriptCraft): new name and
  stag-beetle icon; Linux AppImage, .deb, .rpm and .tar.gz; ARM64 builds for Linux and Windows and
  single-architecture macOS builds; Russian user guide on the website; Rust 1.100, egui 0.37.
