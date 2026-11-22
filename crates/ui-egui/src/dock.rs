//! Docking the window's side panels: the Navigator and the Story Map can sit on any side of the
//! window (left, right, top or bottom), chosen with View › Dock Navigator / Dock Story Map, the
//! right-click menu of a panel's title, or `ui.dockPanel`. The choice is kept in settings
//! (`[layout]`), together with which panels are open.
//!
//! Layout presets (View › Layout, `ui.layoutPreset`) set the panels, the main view and the split
//! screen for a kind of work in one step, and View › Layout › Reset Layout (`ui.resetLayout`)
//! puts everything back as it was on the first run.
//!
//! This is the first slice of the movable panels (ROADMAP v0.6): panels dragged by their title,
//! stacked as tabs and split come with the docking library.

use egui::{RichText, Ui};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::split::SplitMode;
use crate::{ScriptApp, View};

/// A side of the window a panel docks to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    #[default]
    Left,
    Right,
    Top,
    Bottom,
}

impl Side {
    pub const ALL: [Side; 4] = [Side::Left, Side::Right, Side::Top, Side::Bottom];

    pub fn id(self) -> &'static str {
        match self {
            Side::Left => "left",
            Side::Right => "right",
            Side::Top => "top",
            Side::Bottom => "bottom",
        }
    }

    pub fn parse(s: &str) -> Option<Side> {
        match s.trim().to_ascii_lowercase().as_str() {
            "left" | "west" => Some(Side::Left),
            "right" | "east" => Some(Side::Right),
            "top" | "north" => Some(Side::Top),
            "bottom" | "south" => Some(Side::Bottom),
            _ => None,
        }
    }

    /// The menu label (English; translated through the catalogue).
    pub fn label(self) -> &'static str {
        match self {
            Side::Left => "Left",
            Side::Right => "Right",
            Side::Top => "Top",
            Side::Bottom => "Bottom",
        }
    }

    /// Left or right: the panel is a column, sized by its width.
    pub fn is_column(self) -> bool {
        matches!(self, Side::Left | Side::Right)
    }

    /// An egui panel on this side. The id carries the side, so a column's width is never
    /// reused as a row's height when the panel moves.
    pub fn panel(self, name: &str) -> egui::Panel {
        let id = format!("{name}@{}", self.id());
        match self {
            Side::Left => egui::Panel::left(id),
            Side::Right => egui::Panel::right(id),
            Side::Top => egui::Panel::top(id),
            Side::Bottom => egui::Panel::bottom(id),
        }
    }
}

/// A panel that docks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DockPanel {
    Navigator,
    StoryMap,
}

impl DockPanel {
    pub const ALL: [DockPanel; 2] = [DockPanel::Navigator, DockPanel::StoryMap];

    pub fn id(self) -> &'static str {
        match self {
            DockPanel::Navigator => "navigator",
            DockPanel::StoryMap => "storyMap",
        }
    }

    pub fn parse(s: &str) -> Option<DockPanel> {
        match s.trim().to_ascii_lowercase().replace(['-', '_', ' '], "").as_str() {
            "navigator" | "nav" => Some(DockPanel::Navigator),
            "storymap" | "map" => Some(DockPanel::StoryMap),
            _ => None,
        }
    }

    /// The View menu's submenu label.
    pub fn menu_label(self) -> &'static str {
        match self {
            DockPanel::Navigator => "Dock Navigator",
            DockPanel::StoryMap => "Dock Story Map",
        }
    }
}

/// `[layout]` in settings: where each panel docks and whether it is open.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Layout {
    pub navigator: Side,
    pub story_map: Side,
    /// The Navigator is shown.
    pub navigator_open: bool,
    /// The Story Map is shown.
    pub story_map_open: bool,
}

impl Default for Layout {
    fn default() -> Self {
        Layout { navigator: Side::Left, story_map: Side::Bottom, navigator_open: true, story_map_open: false }
    }
}

impl Layout {
    pub fn side(&self, p: DockPanel) -> Side {
        match p {
            DockPanel::Navigator => self.navigator,
            DockPanel::StoryMap => self.story_map,
        }
    }

    pub fn set(&mut self, p: DockPanel, side: Side) {
        match p {
            DockPanel::Navigator => self.navigator = side,
            DockPanel::StoryMap => self.story_map = side,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({"navigator": self.navigator.id(), "storyMap": self.story_map.id()})
    }
}

/// A ready-made layout for a kind of work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preset {
    /// The page and nothing else.
    Writing,
    /// Index cards with the Navigator and the Story Map.
    Structure,
    /// The page beside the reports, with the Navigator.
    Production,
}

impl Preset {
    pub const ALL: [Preset; 3] = [Preset::Writing, Preset::Structure, Preset::Production];

    pub fn id(self) -> &'static str {
        match self {
            Preset::Writing => "writing",
            Preset::Structure => "structure",
            Preset::Production => "production",
        }
    }

    pub fn parse(s: &str) -> Option<Preset> {
        match s.trim().to_ascii_lowercase().as_str() {
            "writing" | "write" => Some(Preset::Writing),
            "structure" => Some(Preset::Structure),
            "production" => Some(Preset::Production),
            _ => None,
        }
    }

    /// The menu label (English; translated through the catalogue).
    pub fn label(self) -> &'static str {
        match self {
            Preset::Writing => "Writing",
            Preset::Structure => "Structure",
            Preset::Production => "Production",
        }
    }

    /// The panels of this preset.
    pub fn layout(self) -> Layout {
        let base = Layout::default();
        match self {
            Preset::Writing => Layout { navigator_open: false, story_map_open: false, ..base },
            Preset::Structure => Layout { navigator_open: true, story_map_open: true, ..base },
            Preset::Production => Layout { navigator_open: true, story_map_open: false, ..base },
        }
    }

    /// The main view and the second pane (`None`: one pane).
    pub fn views(self) -> (View, Option<View>) {
        match self {
            Preset::Writing => (View::Page, None),
            Preset::Structure => (View::Cards, None),
            Preset::Production => (View::Page, Some(View::Reports)),
        }
    }

    /// The preset the window shows now, if it matches one exactly.
    pub fn current(app: &ScriptApp) -> Option<Preset> {
        Preset::ALL.into_iter().find(|p| {
            let (main, second) = p.views();
            let shown = if app.split.on() { Some(app.split.second) } else { None };
            let want = p.layout();
            // The open flags reach `prefs` once a frame: read them from the window.
            app.prefs.layout.navigator == want.navigator
                && app.prefs.layout.story_map == want.story_map
                && app.navigator == want.navigator_open
                && app.story_map == want.story_map_open
                && app.view == main
                && shown == second
                && (second.is_none() || app.split.mode == SplitMode::SideBySide)
        })
    }
}

/// Put panels, the main view and the split screen as `layout` and `views` say.
pub fn apply(app: &mut ScriptApp, layout: Layout, views: (View, Option<View>)) {
    app.navigator = layout.navigator_open;
    app.story_map = layout.story_map_open;
    app.prefs.layout = layout;
    let (main, second) = views;
    app.view = main;
    if main == View::Page {
        app.session.speed = false;
    }
    app.split.focus = 0;
    match second {
        Some(v) => {
            app.split.mode = SplitMode::SideBySide;
            app.split.second = v;
            app.split.canvas.zoom = app.canvas.zoom;
        }
        None => app.split.mode = SplitMode::Off,
    }
    app.canvas.scroll_to_caret = true;
}

/// View › Layout: the presets and Reset Layout.
pub fn layout_menu(app: &mut ScriptApp, ui: &mut Ui) {
    let lang = app.prefs.appearance.language;
    let cur = Preset::current(app);
    for p in Preset::ALL {
        if ui.radio(cur == Some(p), lang.text(p.label())).clicked() {
            app.run("ui.layoutPreset", json!({"preset": p.id()}));
            ui.close();
        }
    }
    ui.separator();
    if ui.button(lang.text("Reset Layout")).clicked() {
        app.run("ui.resetLayout", json!({}));
        ui.close();
    }
}

/// The four sides as radio buttons, for the View menu and a panel title's right-click menu.
pub fn side_choices(app: &mut ScriptApp, ui: &mut Ui, p: DockPanel) {
    let lang = app.prefs.appearance.language;
    let cur = app.prefs.layout.side(p);
    for side in Side::ALL {
        if ui.radio(cur == side, lang.text(side.label())).clicked() {
            app.run("ui.dockPanel", json!({"panel": p.id(), "side": side.id()}));
            ui.close();
        }
    }
}

/// A panel's title: small capitals, with a right-click menu to dock it elsewhere.
pub fn title(app: &mut ScriptApp, ui: &mut Ui, p: DockPanel, text: &str) {
    let t = app.tokens();
    let lang = app.prefs.appearance.language;
    let r = ui
        .add(egui::Label::new(RichText::new(text).size(11.0).strong().color(t.text_soft)).sense(egui::Sense::click()))
        .on_hover_text(lang.text("Right-click to dock this panel to another side"));
    r.context_menu(|ui| {
        ui.label(RichText::new(lang.text("Dock To")).strong());
        side_choices(app, ui, p);
    });
}
