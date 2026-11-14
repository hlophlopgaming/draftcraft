# DraftCraft release notes

The section for each version is also the description of its GitHub release
(`.github/workflows/release.yml` copies the `## vX.Y.Z` section).

## v0.5.1

DraftCraft is a fork of [ScriptCraft](https://github.com/NoJuanNobody/scriptcraft): a free,
open-source screenwriting app in pure Rust. This is its first release under the new name.

### Downloads

| System | File |
|---|---|
| **Windows** (Intel/AMD) | [`draftcraft-windows-x64.zip`](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.1/draftcraft-windows-x64.zip) |
| **Windows** (ARM, e.g. Snapdragon) | [`draftcraft-windows-arm64.zip`](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.1/draftcraft-windows-arm64.zip) |
| **macOS** (any Mac) | [`draftcraft-macos-universal.zip`](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.1/draftcraft-macos-universal.zip) · smaller: [Apple Silicon](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.1/draftcraft-macos-arm64.zip), [Intel](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.1/draftcraft-macos-x86_64.zip) |
| **Linux x86_64** | [AppImage](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.1/draftcraft-linux-x86_64.AppImage) · [.deb](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.1/draftcraft-linux-x86_64.deb) · [.rpm](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.1/draftcraft-linux-x86_64.rpm) · [.tar.gz](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.1/draftcraft-linux-x86_64.tar.gz) |
| **Linux ARM64** | [AppImage](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.1/draftcraft-linux-aarch64.AppImage) · [.deb](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.1/draftcraft-linux-aarch64.deb) · [.rpm](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.1/draftcraft-linux-aarch64.rpm) · [.tar.gz](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.1/draftcraft-linux-aarch64.tar.gz) |
| **Browser** | [open the web app](https://hlophlopgaming.github.io/draftcraft/app/) or self-host [`draftcraft-web.zip`](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.1/draftcraft-web.zip) |

Checksums: [`SHA256SUMS.txt`](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.1/SHA256SUMS.txt). The macOS builds are not notarized yet: the first
time, right-click `DraftCraft.app` and choose **Open**. The [user guide](https://hlophlopgaming.github.io/draftcraft/guide/)
(in Russian) explains installation on each system.

### New

- **DraftCraft name and icon**: the engraved stag beetle on a sand tile, in the window, the
  macOS app, Linux menus and the web app.
- **Linux packages**: AppImage, `.deb`, `.rpm` and `.tar.gz`, with a desktop entry, icon and
  AppStream metadata.
- **ARM builds**: Linux ARM64, Windows ARM64, and separate Apple Silicon and Intel macOS builds
  next to the universal one.
- **User guide** in Russian on the website.

### Under the hood

- Rust 1.100 (pinned in `rust-toolchain.toml`), egui/eframe 0.37, quick-xml 0.42, rfd 0.18 and
  current versions of every other dependency.
- egui 0.37 moved its symbol fonts out of the default set; DraftCraft turns them back on so arrows
  and marks in the interface still draw.

## v0.5.0

**DraftCraft v0.5.0** is the first full multi-platform release of DraftCraft, bringing together milestones **v0.1 through v0.5** into pre-built packages for **macOS (Universal)**, **Windows (x64)**, **Linux (x86_64)**, and the **Web (WebAssembly)**.

---

### Downloads & Artifacts

| Platform | Artifact | Contents |
|---|---|---|
| **macOS (Universal — Apple Silicon & Intel)** | [`draftcraft-macos-universal.zip`](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.0/draftcraft-macos-universal.zip) | `DraftCraft.app` bundle + `draftcraft-cli`, licenses, and README |
| **Windows (x64)** | [`draftcraft-windows-x64.zip`](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.0/draftcraft-windows-x64.zip) | `draftcraft.exe` + `draftcraft-cli.exe`, licenses, and README ([Windows Setup Guide](docs/windows-setup.md)) |
| **Linux (x86_64 tarball)** | [`draftcraft-linux-x86_64.tar.gz`](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.0/draftcraft-linux-x86_64.tar.gz) | `draftcraft` + `draftcraft-cli`, licenses, and README |
| **Linux (Debian / Ubuntu `.deb`)** | [`draftcraft-linux-x86_64.deb`](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.0/draftcraft-linux-x86_64.deb) | `/usr/bin/draftcraft`, `/usr/bin/draftcraft-cli`, and `.desktop` launcher |
| **Web (WASM static bundle)** | [`draftcraft-web.zip`](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.0/draftcraft-web.zip) | Self-hostable WebAssembly build (also live at [hlophlopgaming.github.io/draftcraft/app/](https://hlophlopgaming.github.io/draftcraft/app/)) |
| **Checksums** | [`SHA256SUMS.txt`](https://github.com/hlophlopgaming/draftcraft/releases/download/v0.5.0/SHA256SUMS.txt) | SHA-256 digests for all release archives |

---

### Highlights in v0.5.0

#### 1. Fidelity, Import & Native Printing (`v0.5` — PR #49)
- **Import from Word (`.docx`), Celtx (`.celtx`), and PDF (`.pdf`)** (#31): Dedicated `draftcraft-import` crate and `file.import` engine command with automatic screenplay element classification (scene headings, character cues, parentheticals, dialogue, transitions, dual dialogue, and title pages).
- **Direct System & Browser Printing** (#43): `print.printers` and `print.send` commands, native printer enumeration and spooling (`lp` on macOS/Linux, PowerShell/WinSpool on Windows), copy count, PDF viewer handoff, and hidden `<iframe>` print dialog in the browser.
- **Full Final Draft (`.fdx`) Title Page & Compatibility** (#30, #42): Separate round-tripping for all title-page fields (`title`, `credit`, `author`, `source`, `draft_date`, `contact`, `copyright`, `notes`), free-form title-page preservation, and native Final Draft XML tags (`<HeaderAndFooter>`, `<Revision>`, `<PageLayout>`, `<ScriptNote>`, `<SmartType>`, `<SpellCheck>`, `<TextState>`, `<Cast>`).
- **Fountain Fidelity** (#29): Preserves `Shot` (`!!`) and `General` (`@@`) elements across Fountain round-trips.
- **Locked Page Labels in Reports & Dual Dialogue Wrapping** (#41, #45): Reports now display printed page labels (e.g. `12A`) when pages are locked, and long character cues with `(CONT'D)` wrap cleanly within their dual-dialogue column.

#### 2. Russian Localization & Cyrillic Screenplay Support (PR #48)
- **Courier Badi v1.010 Embedded Typeface**: Replaces Courier Prime with Courier Badi (SIL OFL 1.1) across all four faces (Regular, Bold, Italic, Bold Italic), adding full Cyrillic coverage (including `Ё`/`ё`) at the same 10-cpi / 6-lpi screenplay metrics in both the live editor and exported PDFs.
- **Russian UI Localization (`View → Language → Русский`)**: Complete Russian translation catalog (`crates/ui-egui/locales/ru.json`) covering menus, toolbars, dialogs, cards, reports, and status bar messages.
- **Cyrillic Screenplay Conventions**: Recognizes Russian scene heading prefixes (`ИНТ.`, `НАТ.`, `ИНТ./НАТ.`, `ЭКСТ.`), time-of-day suffixes, transitions (`ЗТМ:`, `СКЛЕЙКА:`), and localized continuation markers (`(ДАЛЬШЕ)`, `(ПРОД.)`).

#### 3. Structure, Collaboration, Speech & Draft Comparison (`v0.4` — PR #47)
- **Acts, Sequences, Outline View & Story Map** (#21, #23): Hierarchical structure points with drag-and-drop scene reorganization and a proportional page-scaled story map.
- **2D Beat Board** (#22): Free-form canvas of colour-coded beats linked to scenes (`beat.*` commands).
- **Track Changes & Draft Comparison** (#24): Per-author tracked insertions and deletions with accept/reject workflows, plus side-by-side/unified draft comparison (`draft.compare` and `draftcraft-cli compare`).
- **Real-Time LAN Collaboration** (#25): Deterministic CRDT/oplog session sharing over TCP (`collab.*`) with live remote carets and author attribution ([docs/collaboration.md](docs/collaboration.md)).
- **Read Aloud & Dictation** (#27): Table reads with per-character system TTS voices and speech-to-script dictation.
- **Screenplay Templates, Bookmarks & Split Screen** (#26, #28): Built-in templates for Feature Film, TV Single-Camera, TV Multi-Camera, and Stage Play; quick bookmarks and dual split-screen views.

#### 4. Production Workflows (`v0.3`) & Writing Comfort (`v0.2`)
- **Production Tools** (#13–#20): Coloured revision sets with margin asterisks, locked pages with `A`/`B` numbering, omitted scenes, locked scene numbers, custom headers/footers, A4 paper support, script notes, alternate dialogue lines, and 17-category production breakdown tagging & reports.
- **Writing Comfort** (#1–#12): Inline Bold/Italic/Underline, offline spell check with user dictionary, customizable element settings, SmartType autocomplete, index card colours/tags and drag reorder, autosave with crash recovery and backups, speed view, and `craft-settings` configuration (`config.toml`).

#### 5. Interactive Explainer, Windows Setup Guide & GitHub Pages (PRs #50, #53, Issue #54)
- **Landing Page, Live Web App & Interactive Explainer**: Hosted at [hlophlopgaming.github.io/draftcraft/](https://hlophlopgaming.github.io/draftcraft/) with the browser app at [`/app/`](https://hlophlopgaming.github.io/draftcraft/app/) and interactive architecture tour + 60-second video at [`/explainer/`](https://hlophlopgaming.github.io/draftcraft/explainer/).
- **Windows Setup Guide**: Step-by-step instructions and troubleshooting in [docs/windows-setup.md](docs/windows-setup.md).

---

### Headline Numbers in v0.5.0
- **Final Draft 13 Parity**: **63 implemented · 0 partial · 5 not yet** ([docs/parity-checklist.md](docs/parity-checklist.md))
- **Command Registry**: **191 engine commands + 53 window commands** accessible from UI, keyboard shortcuts, `draftcraft-cli`, JSON control socket, and MCP (`draftcraft-mcp`)
- **Test Suite**: **291 automated tests** across 18 workspace crates (~38.7k lines of Rust, `unsafe_code = "forbid"`)
