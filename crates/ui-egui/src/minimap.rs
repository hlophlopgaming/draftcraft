//! The scene minimap (redesign, direction "Focus"): a quiet column of the script's scenes at
//! the right edge of the page view. Each scene is a short line, as long as the scene is on the
//! page, followed by its number and heading; the scene with the caret is in the accent colour.
//! Click a scene to go there. View › Scene Minimap (`ui.toggleMinimap`) shows or hides it, and
//! it steps aside on its own when the window is too narrow to keep the page clear.

use egui::{Align2, FontId, Id, Rect, RichText, Sense, Stroke, Ui, pos2, vec2};
use serde_json::json;

use crate::theme::{space, type_scale};
use crate::{ScriptApp, View};

/// Width of the minimap column, points.
pub const WIDTH: f32 = 210.0;
/// Height of one scene row, points.
const ROW_H: f32 = 24.0;
/// The scene line's length range, points: the longest scene gets `LINE_MAX`.
const LINE_MIN: f32 = 8.0;
const LINE_MAX: f32 = 34.0;

/// Egui temp-data id of the scene rows' rectangles (`Vec<Rect>`, one per scene) drawn last frame.
pub fn rows_id() -> Id {
    Id::unique("minimap-rows")
}

/// Whether the minimap fits beside the page: the page view is showing (one pane), the minimap
/// is on, and the desk is wide enough for the page plus the column.
pub fn visible(app: &mut ScriptApp, available_width: f32) -> bool {
    if !app.minimap || app.split.on() || app.view != View::Page {
        return false;
    }
    let page = app.session.layout().page_width() * app.canvas.zoom;
    available_width >= page + 2.0 * space::XXL + WIDTH
}

/// Length of a scene's line for `eighths` of a page, against the longest scene.
pub fn line_len(eighths: usize, longest: usize) -> f32 {
    let f = if longest == 0 { 0.0 } else { (eighths.min(longest) as f32 / longest as f32).sqrt() };
    LINE_MIN + (LINE_MAX - LINE_MIN) * f
}

/// The minimap docked at the right edge of the window, on the desk colour.
pub fn panel(app: &mut ScriptApp, ui: &mut Ui) {
    if !visible(app, ui.available_width()) {
        ui.ctx().data_mut(|d| d.remove::<Vec<Rect>>(rows_id()));
        return;
    }
    let t = app.tokens();
    egui::Panel::right("minimap")
        .resizable(false)
        .exact_size(WIDTH)
        .show_separator_line(false)
        .frame(egui::Frame::NONE.fill(t.desk).inner_margin(egui::Margin { left: 8, right: 16, top: 24, bottom: 12 }))
        .show(ui, |ui| column(app, ui));
}

fn column(app: &mut ScriptApp, ui: &mut Ui) {
    let lang = app.prefs.appearance.language;
    let t = app.tokens();
    let l = app.session.layout();
    let scenes = draftcraft_engine::reports::scenes(&app.session.script, &l);
    let cur = app.session.script.scene_of(app.session.sel.focus.para);
    ui.horizontal(|ui| {
        ui.add_space(LINE_MAX + space::S);
        ui.label(RichText::new(lang.text("SCENES")).size(type_scale::CAPTION).strong().color(t.text_soft));
    });
    ui.add_space(space::XS);
    if scenes.is_empty() {
        ui.label(RichText::new(lang.text("No scenes yet")).size(type_scale::SMALL).color(t.text_soft));
        return;
    }
    let longest = scenes.iter().map(|s| s.eighths).max().unwrap_or(0);
    // Keep the current scene in sight when the caret moves to another one.
    let last_id = Id::unique("minimap-last-scene");
    let moved = ui.ctx().data(|d| d.get_temp::<Option<usize>>(last_id)) != Some(cur);
    ui.ctx().data_mut(|d| d.insert_temp(last_id, cur));
    let mut rows = Vec::with_capacity(scenes.len());
    let mut go = None;
    egui::ScrollArea::vertical().auto_shrink([false, false]).scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden).show(
        ui,
        |ui| {
            for (i, sc) in scenes.iter().enumerate() {
                let on = cur == Some(i);
                let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), ROW_H), Sense::click());
                rows.push(r);
                let hot = resp.hovered();
                let p = ui.painter();
                let len = line_len(sc.eighths, longest);
                let y = r.center().y;
                let line_right = r.left() + LINE_MAX;
                let colour = if on {
                    t.accent
                } else if hot {
                    t.text
                } else {
                    t.chrome_line
                };
                p.line_segment([pos2(line_right - len, y), pos2(line_right, y)], Stroke::new(if on { 3.0 } else { 2.0 }, colour));
                let num = sc.number.clone().unwrap_or_else(|| sc.scene.to_string());
                let text_left = line_right + space::S;
                let chars = ((r.right() - text_left) / 6.4).max(4.0) as usize;
                p.text(
                    pos2(text_left, y),
                    Align2::LEFT_CENTER,
                    crate::chrome::elide(&format!("{num}. {}", sc.heading), chars),
                    FontId::proportional(type_scale::SMALL),
                    if on {
                        t.accent
                    } else if hot {
                        t.text
                    } else {
                        t.text_soft
                    },
                );
                if on && moved {
                    resp.scroll_to_me(Some(egui::Align::Center));
                }
                let resp = resp.on_hover_text(lang.format(
                    "{heading}\nPage {page} · {length}",
                    &[
                        ("heading", sc.heading.clone()),
                        ("page", sc.page_label.clone()),
                        ("length", draftcraft_engine::reports::eighths_label(sc.eighths)),
                    ],
                ));
                if resp.clicked() {
                    go = Some(i);
                }
            }
        },
    );
    ui.ctx().data_mut(|d| d.insert_temp(rows_id(), rows));
    if let Some(i) = go {
        app.run("scene.goto", json!({"index": i}));
        crate::split::show_page(app);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_lines_grow_with_length_and_stay_in_range() {
        assert_eq!(line_len(0, 0), LINE_MIN);
        assert_eq!(line_len(0, 16), LINE_MIN);
        assert_eq!(line_len(16, 16), LINE_MAX);
        assert!(line_len(4, 16) < line_len(8, 16));
        // Hostile numbers stay inside the range.
        assert_eq!(line_len(usize::MAX, 16), LINE_MAX);
        assert_eq!(line_len(usize::MAX, usize::MAX), LINE_MAX);
    }
}
