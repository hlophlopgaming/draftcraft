//! Docking the window's side panels: the Navigator and the Story Map can sit on any side of the
//! window (left, right, top or bottom), chosen with View › Dock Navigator / Dock Story Map, the
//! right-click menu of a panel's title, by dragging the panel by its title, or `ui.dockPanel`.
//! The choice is kept in settings (`[layout]`).
//!
//! Dragging: the title (with its grip) starts a drag; while it lasts, [`overlay`] draws a
//! four-way target in the middle of the window and a preview of where the panel will land.
//! Dropping on a target button, or near an edge of the window, runs `ui.dockPanel`; dropping
//! anywhere else, or Esc, leaves the panel where it was.
//!
//! Part of the movable panels (ROADMAP v0.6): panels stacked as tabs and split come with the
//! docking library.

use egui::{Align2, Color32, CursorIcon, FontId, Id, LayerId, Order, Pos2, Rect, RichText, Sense, Shape, Stroke, StrokeKind, Ui, pos2, vec2};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::ScriptApp;
use crate::theme::{radius, space, type_scale};
use egui::CornerRadius;

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

    /// What dropping here does, for the drag preview (English; translated).
    pub fn drop_label(self) -> &'static str {
        match self {
            Side::Left => "Dock on the left",
            Side::Right => "Dock on the right",
            Side::Top => "Dock at the top",
            Side::Bottom => "Dock at the bottom",
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

    /// The panel's title (English; translated through the catalogue).
    pub fn title(self) -> &'static str {
        match self {
            DockPanel::Navigator => "NAVIGATOR",
            DockPanel::StoryMap => "STORY MAP",
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

/// `[layout]` in settings: where each panel docks.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Layout {
    pub navigator: Side,
    pub story_map: Side,
}

impl Default for Layout {
    fn default() -> Self {
        Layout { navigator: Side::Left, story_map: Side::Bottom }
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

/// A panel's title: a grip and small capitals. Drag it to dock the panel to another side;
/// right-click it for the same choice as a menu.
pub fn title(app: &mut ScriptApp, ui: &mut Ui, p: DockPanel, text: &str) {
    let t = app.tokens();
    let lang = app.prefs.appearance.language;
    let dragging = app.dock_drag == Some(p);
    let color = if dragging { t.accent } else { t.text_soft };
    let (grip, _) = ui.allocate_exact_size(vec2(8.0, 14.0), Sense::hover());
    paint_grip(ui.painter(), grip, color);
    let label = ui.add(egui::Label::new(RichText::new(text).size(11.0).strong().color(color)).selectable(false).sense(Sense::click_and_drag()));
    // The grip drags too: one response over both.
    let r = label.union(ui.interact(grip, Id::unique(("dock-grip", p.id())), Sense::click_and_drag()));
    if r.drag_started() {
        app.dock_drag = Some(p);
    }
    if r.hovered() && app.dock_drag.is_none() {
        ui.ctx().set_cursor_icon(CursorIcon::Grab);
    }
    let r = r.on_hover_text(lang.text("Drag to dock this panel to another side, or right-click"));
    r.context_menu(|ui| {
        ui.label(RichText::new(lang.text("Dock To")).strong());
        side_choices(app, ui, p);
    });
}

/// Six dots in two columns: "this can be dragged".
fn paint_grip(painter: &egui::Painter, r: Rect, color: Color32) {
    for row in 0..3 {
        for col in 0..2 {
            let c = pos2(r.left() + 2.0 + col as f32 * 4.0, r.center().y - 4.0 + row as f32 * 4.0);
            painter.circle_filled(c, 1.1, color);
        }
    }
}

/// Size of a button of the four-way target, and the gap between them.
const TARGET: f32 = 34.0;
const TARGET_GAP: f32 = 6.0;

/// The four buttons of the drop target, in the middle of `screen`.
pub fn target_rects(screen: Rect) -> [(Side, Rect); 4] {
    let c = screen.center();
    let step = TARGET + TARGET_GAP;
    let at = |dx: f32, dy: f32| Rect::from_center_size(c + vec2(dx, dy), vec2(TARGET, TARGET));
    [(Side::Left, at(-step, 0.0)), (Side::Right, at(step, 0.0)), (Side::Top, at(0.0, -step)), (Side::Bottom, at(0.0, step))]
}

/// How close to an edge of the window a drop docks to that edge.
fn edge_band(screen: Rect) -> f32 {
    (screen.width().min(screen.height()) * 0.12).clamp(32.0, 120.0)
}

/// The side a drop at `pos` docks to: a button of the four-way target, or the nearest edge when
/// `pos` is within a band along it; `None` elsewhere (the drop changes nothing).
pub fn drop_side(screen: Rect, pos: Pos2) -> Option<Side> {
    if !pos.x.is_finite() || !pos.y.is_finite() {
        return None;
    }
    if let Some((side, _)) = target_rects(screen).into_iter().find(|(_, r)| r.expand(TARGET_GAP / 2.0).contains(pos)) {
        return Some(side);
    }
    if !screen.contains(pos) {
        return None;
    }
    let band = edge_band(screen);
    let d = [
        (Side::Left, pos.x - screen.left()),
        (Side::Right, screen.right() - pos.x),
        (Side::Top, pos.y - screen.top()),
        (Side::Bottom, screen.bottom() - pos.y),
    ];
    d.into_iter().filter(|(_, d)| *d <= band).min_by(|a, b| a.1.total_cmp(&b.1)).map(|(s, _)| s)
}

/// Where a panel docked to `side` would sit in `screen`, for the drag preview.
pub fn preview_rect(screen: Rect, p: DockPanel, side: Side) -> Rect {
    let size: f32 = match (p, side.is_column()) {
        (DockPanel::Navigator, true) => 280.0,
        (DockPanel::Navigator, false) => 240.0,
        (DockPanel::StoryMap, true) => 360.0,
        (DockPanel::StoryMap, false) => 130.0,
    };
    let w = size.min(screen.width() / 2.0);
    let h = size.min(screen.height() / 2.0);
    match side {
        Side::Left => Rect::from_min_max(screen.min, pos2(screen.left() + w, screen.bottom())),
        Side::Right => Rect::from_min_max(pos2(screen.right() - w, screen.top()), screen.max),
        Side::Top => Rect::from_min_max(screen.min, pos2(screen.right(), screen.top() + h)),
        Side::Bottom => Rect::from_min_max(pos2(screen.left(), screen.bottom() - h), screen.max),
    }
}

/// While a panel is being dragged: the drop target, the preview, and the drop itself.
pub fn overlay(app: &mut ScriptApp, ctx: &egui::Context) {
    let Some(p) = app.dock_drag else { return };
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        app.dock_drag = None;
        return;
    }
    // The area under the toolbar and above the status bar, where panels dock.
    let screen = if app.dock_area.is_positive() { app.dock_area } else { ctx.content_rect() };
    let (pos, down) = ctx.input(|i| (i.pointer.latest_pos(), i.pointer.primary_down()));
    let side = pos.and_then(|pos| drop_side(screen, pos));
    if !down {
        app.dock_drag = None;
        if let Some(side) = side {
            app.run("ui.dockPanel", serde_json::json!({"panel": p.id(), "side": side.id()}));
        }
        return;
    }
    ctx.set_cursor_icon(CursorIcon::Grabbing);
    let t = app.tokens();
    let lang = app.prefs.appearance.language;
    let painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::unique("dock-drag")));
    let accent_fill = t.accent.gamma_multiply(0.12);
    let current = app.prefs.layout.side(p);
    // The preview: a dashed outline where the panel will land, with what the drop does.
    if let Some(side) = side {
        let r = preview_rect(screen, p, side).shrink(space::S);
        painter.rect_filled(r, CornerRadius::same(radius::M), accent_fill);
        dashed_rect(&painter, r, Stroke::new(1.5, t.accent));
        painter.text(r.center(), Align2::CENTER_CENTER, lang.text(side.drop_label()), FontId::proportional(type_scale::BODY), t.accent);
    }
    // The four-way target on a floating card.
    let rects = target_rects(screen);
    let card = Rect::from_center_size(screen.center(), vec2(3.0 * TARGET + 4.0 * TARGET_GAP, 3.0 * TARGET + 4.0 * TARGET_GAP));
    let shadow = egui::epaint::Shadow { offset: [0, 4], blur: 16, spread: 0, color: t.shadow };
    painter.add(shadow.as_shape(card, CornerRadius::same(radius::L)));
    painter.rect_filled(card, CornerRadius::same(radius::L), t.pop);
    painter.rect_stroke(card, CornerRadius::same(radius::L), Stroke::new(1.0, t.chrome_line), StrokeKind::Inside);
    for (s, r) in rects {
        let hot = side == Some(s);
        let fill = if hot { t.accent } else { t.field };
        let ink = if hot { t.on_accent } else { t.text_soft };
        painter.rect_filled(r, CornerRadius::same(radius::S), fill);
        if s == current && !hot {
            painter.rect_stroke(r, CornerRadius::same(radius::S), Stroke::new(1.5, t.accent), StrokeKind::Inside);
        }
        // A window with the docked strip filled in on that side.
        let w = r.shrink(9.0);
        painter.rect_stroke(w, CornerRadius::same(2), Stroke::new(1.4, ink), StrokeKind::Inside);
        let strip = match s {
            Side::Left => Rect::from_min_max(w.min, pos2(w.left() + w.width() * 0.4, w.bottom())),
            Side::Right => Rect::from_min_max(pos2(w.right() - w.width() * 0.4, w.top()), w.max),
            Side::Top => Rect::from_min_max(w.min, pos2(w.right(), w.top() + w.height() * 0.4)),
            Side::Bottom => Rect::from_min_max(pos2(w.left(), w.bottom() - w.height() * 0.4), w.max),
        };
        painter.rect_filled(strip, CornerRadius::same(2), ink);
    }
    // The panel's name follows the pointer.
    if let Some(pos) = pos {
        let galley = painter.layout_no_wrap(lang.text(p.title()).to_string(), FontId::proportional(type_scale::SMALL), t.text);
        let size = galley.size() + vec2(2.0 * space::M, 2.0 * space::S);
        // Beside the pointer, flipped to its left near the right edge of the window.
        let win = ctx.content_rect();
        let x = if pos.x + 14.0 + size.x > win.right() { pos.x - 14.0 - size.x } else { pos.x + 14.0 };
        let chip = Rect::from_min_size(pos2(x, pos.y + 10.0), size);
        painter.rect_filled(chip, CornerRadius::same(radius::S), t.pop);
        painter.rect_stroke(chip, CornerRadius::same(radius::S), Stroke::new(1.0, t.accent), StrokeKind::Inside);
        painter.galley(chip.min + vec2(space::M, space::S), galley, t.text);
    }
    // A hint at the bottom of the window.
    let hint = painter.layout_no_wrap(
        lang.text("Drop on a side or a button to dock the panel · Esc cancels").to_string(),
        FontId::proportional(type_scale::SMALL),
        t.text_soft,
    );
    let pill = Rect::from_center_size(pos2(screen.center().x, screen.bottom() - 28.0), hint.size() + vec2(2.0 * space::L, 2.0 * space::S));
    painter.rect_filled(pill, CornerRadius::same(u8::MAX), t.pop);
    painter.rect_stroke(pill, CornerRadius::same(u8::MAX), Stroke::new(1.0, t.chrome_line), StrokeKind::Inside);
    painter.galley(pill.min + vec2(space::L, space::S), hint, t.text_soft);
    ctx.request_repaint();
}

fn dashed_rect(painter: &egui::Painter, r: Rect, stroke: Stroke) {
    let pts = [r.left_top(), r.right_top(), r.right_bottom(), r.left_bottom(), r.left_top()];
    painter.extend(Shape::dashed_line(&pts, stroke, 8.0, 5.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn screen() -> Rect {
        Rect::from_min_size(Pos2::ZERO, vec2(1440.0, 900.0))
    }

    #[test]
    fn drop_on_a_target_button_docks_to_its_side() {
        for (side, r) in target_rects(screen()) {
            assert_eq!(drop_side(screen(), r.center()), Some(side));
        }
    }

    #[test]
    fn drop_near_an_edge_docks_there_and_elsewhere_does_nothing() {
        let s = screen();
        assert_eq!(drop_side(s, pos2(20.0, 450.0)), Some(Side::Left));
        assert_eq!(drop_side(s, pos2(1420.0, 300.0)), Some(Side::Right));
        assert_eq!(drop_side(s, pos2(700.0, 10.0)), Some(Side::Top));
        assert_eq!(drop_side(s, pos2(500.0, 890.0)), Some(Side::Bottom));
        // A corner goes to the nearer edge.
        assert_eq!(drop_side(s, pos2(5.0, 40.0)), Some(Side::Left));
        // The middle of the page, away from the target, and outside the window: nothing.
        assert_eq!(drop_side(s, pos2(400.0, 300.0)), None);
        assert_eq!(drop_side(s, pos2(-50.0, 450.0)), None);
        assert_eq!(drop_side(s, pos2(f32::NAN, 450.0)), None);
        assert_eq!(drop_side(s, pos2(f32::INFINITY, f32::NEG_INFINITY)), None);
    }

    #[test]
    fn tiny_and_degenerate_windows_never_panic() {
        for s in
            [Rect::NOTHING, Rect::EVERYTHING, Rect::from_min_size(Pos2::ZERO, vec2(10.0, 10.0)), Rect::from_min_size(Pos2::ZERO, egui::Vec2::ZERO)]
        {
            for p in DockPanel::ALL {
                for side in Side::ALL {
                    let _ = preview_rect(s, p, side);
                }
            }
            let _ = drop_side(s, pos2(5.0, 5.0));
        }
    }

    #[test]
    fn preview_sits_on_its_side() {
        let s = screen();
        for p in DockPanel::ALL {
            assert!(preview_rect(s, p, Side::Left).center().x < 400.0);
            assert!(preview_rect(s, p, Side::Right).center().x > 1040.0);
            assert!(preview_rect(s, p, Side::Top).center().y < 300.0);
            assert!(preview_rect(s, p, Side::Bottom).center().y > 600.0);
        }
    }
}
