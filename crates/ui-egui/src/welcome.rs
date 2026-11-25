//! The welcome screen (Help › Welcome Screen, `ui.welcome`): the Focus direction's start screen
//! (`docs/design/focus/welcome-*.png`). Continue with the open script, start a new one from a
//! template, open a file, pick a recent script, or find every command and the guide.
//!
//! It is shown at start when the app opens without a file (unless switched off with "Show at
//! start", `appearance.show_welcome` in `config.toml`) and closes once a script is opened or
//! created, with Escape or with "Continue writing".

use draftcraft_doc::Template;
use egui::{Color32, CornerRadius, Rect, RichText, Sense, Stroke, Ui, pos2, vec2};
use serde_json::{Value, json};

use crate::ScriptApp;
use crate::icons::{self, Icon};
use crate::theme::{Tokens, radius, space, type_scale};

/// Where the guide lives (Russian; the README links it too).
pub const GUIDE_URL: &str = "https://hlophlopgaming.github.io/draftcraft/guide/";

/// Commands that put a new or another script in the window, which closes the welcome screen.
pub const CLOSES: &[&str] = &["file.new", "file.open", "file.load", "file.import", "file.sample", "template.new"];

/// One "New" card: the short label and line under it (English keys of the catalogues) and the
/// template it starts from.
pub const NEW_CARDS: [(Template, &str, &str); 4] = [
    (Template::Feature, "Screenplay", "Feature film, US Letter"),
    (Template::TvSingleCamera, "TV drama", "Single camera, teaser and acts"),
    (Template::TvMultiCamera, "TV sitcom", "Multi-camera, scenes lettered"),
    (Template::StagePlay, "Stage play", "Acts, scenes, stage directions"),
];

/// A recent file as the screen lists it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recent {
    pub path: String,
    /// The file name.
    pub name: String,
    /// The folder it is in.
    pub folder: String,
    /// Its format in English (`Final Draft`, `Fountain`, …), a catalogue key.
    pub format: &'static str,
}

/// The recent files, most recent first. Paths are hostile input from `config.toml`: only their
/// last component and extension are read, nothing is opened.
pub fn recent(app: &ScriptApp) -> Vec<Recent> {
    app.prefs
        .files
        .recent
        .iter()
        .map(|p| {
            let path = std::path::Path::new(p);
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or(p).to_string();
            let folder = path.parent().and_then(|f| f.to_str()).unwrap_or("").to_string();
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
            let format = match ext.as_str() {
                "fdx" => "Final Draft",
                "fountain" | "spmd" | "txt" => "Fountain",
                "docx" => "Word",
                "celtx" => "Celtx",
                "pdf" => "PDF",
                _ => "Script",
            };
            Recent { path: p.clone(), name, folder, format }
        })
        .collect()
}

/// `ui.welcome`: `{"open": bool (default: toggle), "atStart": bool}`. Returns whether it is
/// open, whether it shows at start, the recent files and the templates it offers.
pub fn command(app: &mut ScriptApp, p: &Value) -> Value {
    app.welcome = p.get("open").and_then(Value::as_bool).unwrap_or(!app.welcome);
    if let Some(on) = p.get("atStart").and_then(Value::as_bool) {
        app.prefs.appearance.show_welcome = on;
    }
    let recent: Vec<String> = recent(app).into_iter().map(|r| r.path).collect();
    let templates: Vec<&str> = NEW_CARDS.iter().map(|(t, ..)| t.id()).collect();
    json!({"open": app.welcome, "atStart": app.prefs.appearance.show_welcome, "recent": recent, "templates": templates})
}

/// Whether the screen opens at start: the setting is on, nothing was chosen on the command line
/// (`chosen`: a file, `--new`, `--sample` or `--control`) and no recovered script is offered.
pub fn at_start(app: &ScriptApp, chosen: bool) -> bool {
    !chosen && app.prefs.appearance.show_welcome && app.recovery_offer.is_none()
}

/// Draw the screen in the central area. Escape closes it.
pub fn show(app: &mut ScriptApp, ui: &mut Ui) {
    let ctx = ui.ctx().clone();
    if app.palette.is_none() && app.confirm_replace.is_none() && ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
        app.welcome = false;
        return;
    }
    let t = app.tokens();
    let lang = app.prefs.appearance.language;
    let mut run: Option<(String, Value)> = None;
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        let full = ui.available_width();
        let width = (full - 2.0 * space::L).clamp(280.0, 980.0);
        let side = ((full - width) / 2.0).max(0.0);
        // The theme switch in the top corner, as in the mockup.
        ui.horizontal(|ui| {
            ui.add_space((side + width - 30.0).max(0.0));
            let (icon, tip) = if app.dark { (Icon::Sun, "Light Theme") } else { (Icon::Moon, "Dark Theme") };
            if icons::button(ui, icon, lang.text(tip), false).clicked() {
                run = Some(("ui.theme.toggle".into(), json!({})));
            }
        });
        ui.add_space(space::XXL);
        ui.horizontal(|ui| {
            ui.add_space(side);
            ui.vertical(|ui| {
                ui.set_width(width);
                header(app, ui, &t, &mut run);
                ui.add_space(space::XL);
                caption(ui, &t, lang.text("NEW"));
                ui.add_space(space::S);
                new_cards(app, ui, &t, &mut run);
                ui.add_space(space::XL);
                ui.horizontal(|ui| {
                    caption(ui, &t, lang.text("RECENT"));
                    if !app.prefs.files.recent.is_empty() {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if link(ui, &t, lang.text("Clear Recent Files")).clicked() {
                                run = Some(("ui.clearRecent".into(), json!({})));
                            }
                        });
                    }
                });
                ui.add_space(space::S);
                recent_list(app, ui, &t, &mut run);
                ui.add_space(space::L);
                footer(app, ui, &t, &mut run);
                ui.add_space(space::XXL);
            });
        });
    });
    if let Some((id, p)) = run {
        if id == "ui.guide" {
            ctx.open_url(egui::OpenUrl::new_tab(GUIDE_URL));
        } else {
            app.run(&id, p);
        }
    }
}

fn caption(ui: &mut Ui, t: &Tokens, text: &str) {
    ui.label(RichText::new(text).size(type_scale::CAPTION).strong().color(t.text_soft));
}

fn link(ui: &mut Ui, t: &Tokens, text: &str) -> egui::Response {
    ui.add(egui::Label::new(RichText::new(text).size(type_scale::SMALL).color(t.text_soft)).sense(Sense::click()))
        .on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// The greeting, what is open, and "Continue writing".
fn header(app: &mut ScriptApp, ui: &mut Ui, t: &Tokens, run: &mut Option<(String, Value)>) {
    let lang = app.prefs.appearance.language;
    let name = app
        .session
        .path
        .as_ref()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .map_or_else(|| lang.text("Untitled").to_string(), str::to_string);
    let empty = app.session.script.paras.iter().all(|p| p.text.trim().is_empty());
    let line = if empty {
        lang.text("Start a new script, open one, or pick up where you left off.").to_string()
    } else {
        let pages = app.session.layout().script_pages();
        lang.format("{name} is open: {pages} pages.", &[("name", name), ("pages", pages.to_string())])
    };
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(RichText::new(lang.text("Welcome to DraftCraft")).size(type_scale::DISPLAY).strong().color(t.text));
            ui.add_space(space::XS);
            ui.label(RichText::new(line).size(type_scale::BODY).color(t.text_soft));
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let b = egui::Button::new(RichText::new(lang.text("Continue writing")).strong().color(t.on_accent))
                .fill(t.accent)
                .corner_radius(radius::M)
                .min_size(vec2(0.0, 34.0));
            if ui.add(b).clicked() {
                *run = Some(("ui.welcome".into(), json!({"open": false})));
            }
        });
    });
}

/// Templates and "From a file" as cards with a small page drawn on each.
fn new_cards(app: &mut ScriptApp, ui: &mut Ui, t: &Tokens, run: &mut Option<(String, Value)>) {
    let lang = app.prefs.appearance.language;
    let gap = space::M;
    let n = NEW_CARDS.len() as f32 + 1.0;
    // A little slack so rounding never pushes the last card onto a new row.
    let w = ((ui.available_width() - gap * (n - 1.0) - 2.0) / n).floor().max(150.0);
    let opens: Vec<String> =
        draftcraft_engine::io::OPEN_EXTENSIONS.iter().filter(|e| **e != "txt" && **e != "spmd").map(|e| format!(".{e}")).collect();
    let open_line = opens.join(" · ");
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = vec2(gap, gap);
        for (tpl, label, line) in NEW_CARDS {
            let tip = lang.text(tpl.description()).to_string();
            if card(ui, t, w, Some(tpl), Icon::Page, lang.text(label), lang.text(line)).on_hover_text(tip).clicked() {
                *run = Some(("template.new".into(), json!({"template": tpl.id()})));
            }
        }
        if card(ui, t, w, None, Icon::Open, lang.text("From a file"), &open_line).clicked() {
            *run = Some(("ui.open".into(), json!({})));
        }
    });
}

/// One card; returns its click response. Its widget label is the card's name (for tests).
fn card(ui: &mut Ui, t: &Tokens, w: f32, tpl: Option<Template>, icon: Icon, label: &str, line: &str) -> egui::Response {
    let (r, resp) = ui.allocate_exact_size(vec2(w, 178.0), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, label));
    let p = ui.painter();
    let hover = resp.hovered();
    p.rect(r, CornerRadius::from(radius::L), t.card, Stroke::new(1.0, if hover { t.accent } else { t.chrome_line }), egui::StrokeKind::Inside);
    let inner = r.shrink(space::M);
    let preview = Rect::from_min_size(inner.min, vec2(inner.width(), 96.0));
    p.rect(preview, CornerRadius::from(radius::S), t.paper, Stroke::new(1.0, t.paper_edge), egui::StrokeKind::Inside);
    page_lines(p, preview.shrink2(vec2(space::M, space::S + 2.0)), tpl, t.ink_dim);
    let icon_rect = Rect::from_min_size(pos2(inner.left(), preview.bottom() + space::M), vec2(16.0, 16.0));
    icons::paint(ui, icon, icon_rect, if hover { t.accent } else { t.text });
    let font = egui::FontId::proportional(type_scale::BODY);
    p.text(pos2(icon_rect.right() + space::S, icon_rect.center().y), egui::Align2::LEFT_CENTER, label, font, t.text);
    let small = egui::FontId::proportional(type_scale::CAPTION);
    let galley = p.layout(line.to_string(), small, t.text_soft, inner.width());
    p.galley(pos2(inner.left(), icon_rect.bottom() + space::XS + 2.0), galley, t.text_soft);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A few grey lines that sketch the template's page: where action, cues and dialogue sit.
fn page_lines(p: &egui::Painter, r: Rect, tpl: Option<Template>, c: Color32) {
    // (left, right) as fractions of the width, one row each; `None` is an empty row.
    let rows: &[Option<(f32, f32)>] = match tpl {
        Some(Template::Feature) => &[Some((0.0, 0.55)), None, Some((0.0, 1.0)), Some((0.0, 0.7)), None, Some((0.4, 0.6)), Some((0.2, 0.8))],
        Some(Template::TvSingleCamera) => &[Some((0.35, 0.65)), None, Some((0.0, 0.55)), Some((0.0, 1.0)), None, Some((0.4, 0.6)), Some((0.2, 0.75))],
        Some(Template::TvMultiCamera) => &[Some((0.0, 0.6)), Some((0.0, 0.9)), None, Some((0.4, 0.6)), Some((0.2, 0.8)), None, Some((0.2, 0.7))],
        Some(Template::StagePlay) => &[Some((0.4, 0.6)), Some((0.1, 0.9)), Some((0.1, 0.8)), None, Some((0.3, 0.9)), None, Some((0.4, 0.6))],
        None => &[Some((0.0, 1.0)), Some((0.0, 1.0)), Some((0.0, 0.8)), None, Some((0.0, 1.0)), Some((0.0, 0.6)), None],
    };
    let step = r.height() / rows.len() as f32;
    for (i, row) in rows.iter().enumerate() {
        if let Some((a, b)) = row {
            let y = r.top() + step * (i as f32 + 0.5);
            p.line_segment([pos2(r.left() + r.width() * a, y), pos2(r.left() + r.width() * b, y)], Stroke::new(2.0, c));
        }
    }
}

/// The recent files, one row each; a click opens the file.
fn recent_list(app: &mut ScriptApp, ui: &mut Ui, t: &Tokens, run: &mut Option<(String, Value)>) {
    let lang = app.prefs.appearance.language;
    let list = recent(app);
    egui::Frame::NONE.fill(t.card).stroke(Stroke::new(1.0, t.chrome_line)).corner_radius(radius::L).show(ui, |ui| {
        ui.set_width(ui.available_width());
        if list.is_empty() {
            ui.add_space(space::L);
            ui.horizontal(|ui| {
                ui.add_space(space::L);
                ui.label(RichText::new(lang.text("No recent files yet: scripts you open or save appear here.")).color(t.text_soft));
            });
            ui.add_space(space::L);
            return;
        }
        let n = list.len();
        for (i, f) in list.into_iter().enumerate() {
            let missing = missing(&f.path);
            let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 44.0), Sense::click());
            resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, &f.name));
            let p = ui.painter();
            if resp.hovered() {
                p.rect_filled(r.shrink(1.0), CornerRadius::from(radius::M), t.accent_weak);
            }
            if i + 1 < n {
                p.line_segment([pos2(r.left(), r.bottom()), pos2(r.right(), r.bottom())], Stroke::new(1.0, t.chrome_line));
            }
            let x = r.left() + space::L;
            icons::paint(ui, Icon::Page, Rect::from_center_size(pos2(x + 8.0, r.center().y), vec2(16.0, 16.0)), t.text_soft);
            let body = egui::FontId::proportional(type_scale::BODY);
            let small = egui::FontId::proportional(type_scale::SMALL);
            let name_w = (r.width() * 0.4).max(120.0);
            let name = p.layout(f.name.clone(), body, if missing { t.text_soft } else { t.text }, name_w);
            p.galley(pos2(x + 28.0, r.center().y - name.size().y / 2.0), name, t.text);
            let right = if missing { lang.text("Not found").to_string() } else { lang.text(f.format).to_string() };
            p.text(pos2(r.right() - space::L, r.center().y), egui::Align2::RIGHT_CENTER, right, small.clone(), t.text_soft);
            let folder_x = x + 28.0 + name_w + space::M;
            let folder_w = (r.right() - space::L - 110.0 - folder_x).max(0.0);
            if folder_w > 40.0 {
                let mut job = egui::text::LayoutJob::simple_singleline(f.folder.clone(), small, t.text_soft);
                job.wrap = egui::text::TextWrapping::truncate_at_width(folder_w);
                let g = p.layout_job(job);
                p.galley(pos2(folder_x, r.center().y - g.size().y / 2.0), g, t.text_soft);
            }
            let resp = resp.on_hover_text(&f.path).on_hover_cursor(egui::CursorIcon::PointingHand);
            if resp.clicked() {
                *run = Some(("ui.openRecent".into(), json!({"path": f.path})));
            }
        }
    });
}

/// A recent file that is no longer there (never on the web, which has no file system).
fn missing(path: &str) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        !std::path::Path::new(path).exists()
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = path;
        false
    }
}

/// Every command, the guide, the sample script and "Show at start".
fn footer(app: &mut ScriptApp, ui: &mut Ui, t: &Tokens, run: &mut Option<(String, Value)>) {
    let lang = app.prefs.appearance.language;
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = space::L;
        let key = crate::chrome::pretty_key("Mod+K");
        if link(ui, t, &lang.format("{key} — every command", &[("key", key)])).clicked() {
            *run = Some(("ui.commandPalette".into(), json!({"open": true})));
        }
        if link(ui, t, lang.text("Guide")).on_hover_text(GUIDE_URL).clicked() {
            *run = Some(("ui.guide".into(), json!({})));
        }
        if link(ui, t, lang.text("Open Sample Script")).clicked() {
            *run = Some(("file.sample".into(), json!({})));
        }
        let mut on = app.prefs.appearance.show_welcome;
        if ui.checkbox(&mut on, RichText::new(lang.text("Show at start")).size(type_scale::SMALL).color(t.text_soft)).changed() {
            *run = Some(("ui.welcome".into(), json!({"open": true, "atStart": on})));
        }
    });
}
