//! The command palette (View › Command Palette…, Mod+K, `ui.commandPalette`): every command by
//! name, found by typing a few letters of its name, menu or id, in English or the interface
//! language, and run with Enter or a click.
//!
//! Commands that need details (a scene number, a file name, …) and are given none report what
//! they need under the field instead of closing the palette.

use egui::{Key, Modifiers, RichText, Ui};
use serde_json::{Value, json};

use crate::theme::{radius, space, type_scale};
use crate::{ScriptApp, UI_COMMANDS};

/// Left out of the list: the palette itself.
const HIDDEN: &[&str] = &["ui.commandPalette"];

/// The palette's state while it is open.
#[derive(Default, Clone, Debug)]
pub struct Palette {
    pub query: String,
    /// The highlighted row among the matches.
    pub selected: usize,
    /// Give the field the keyboard on the next frame.
    pub focus: bool,
    /// The last command run from the palette failed: (id, message).
    pub error: Option<(String, String)>,
}

impl Palette {
    pub fn new(query: &str) -> Self {
        Palette { query: query.to_string(), selected: 0, focus: true, error: None }
    }
}

/// One command as the palette lists it.
#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub id: String,
    /// The label in the interface language.
    pub label: String,
    /// The English label (also searched).
    pub english: String,
    /// Where it lives in the menus, in the interface language (`Format › Elements`).
    pub location: String,
    pub shortcut: String,
    /// `None` when it can run now, else why not.
    pub disabled: Option<String>,
    /// What the command takes, for agents (`{}` when nothing).
    pub params: String,
}

/// Every command the palette offers, in menu order: the window's own commands first, then the
/// engine's. An engine command with the same name as a window command (Save, Open, Export PDF)
/// is the window one's file-less twin and is left out.
pub fn entries(app: &ScriptApp) -> Vec<Entry> {
    let lang = app.prefs.appearance.language;
    let loc = |l: &str| l.split(" › ").map(|s| lang.text(s)).collect::<Vec<_>>().join(" › ");
    let base = |l: &str| l.trim_end_matches('…').trim().to_ascii_lowercase();
    let mut out: Vec<Entry> = UI_COMMANDS
        .iter()
        .filter(|(id, ..)| !HIDDEN.contains(id))
        .map(|(id, label, location, key)| Entry {
            id: id.to_string(),
            label: lang.text(label).to_string(),
            english: label.to_string(),
            location: loc(location),
            shortcut: key.to_string(),
            disabled: None,
            params: "{}".into(),
        })
        .collect();
    let ui_names: Vec<String> = UI_COMMANDS.iter().map(|(_, l, ..)| base(l)).collect();
    for c in app.session.registry.all().iter() {
        if HIDDEN.contains(&c.id) || ui_names.contains(&base(c.label)) {
            continue;
        }
        out.push(Entry {
            id: c.id.to_string(),
            label: lang.text(c.label).to_string(),
            english: c.label.to_string(),
            location: loc(c.location),
            shortcut: c.shortcut.to_string(),
            disabled: (c.enabled)(&app.session).map(|r| lang.text(r).to_string()),
            params: c.params.to_string(),
        });
    }
    out
}

/// How well an entry matches the query: `None` when some word of the query is in none of its
/// names; lower is better. A label that starts with the query comes first, then labels with a
/// word starting with it, then any other match.
pub fn score(e: &Entry, query: &str) -> Option<u32> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return Some(3);
    }
    let label = e.label.to_lowercase();
    let english = e.english.to_lowercase();
    let hay = format!("{label} {english} {} {}", e.location.to_lowercase(), e.id.to_lowercase());
    if !q.split_whitespace().all(|w| hay.contains(w)) {
        return None;
    }
    let word_start = |s: &str| s.split(|c: char| !c.is_alphanumeric()).any(|w| w.starts_with(q.as_str()));
    Some(if label.starts_with(&q) || english.starts_with(&q) {
        0
    } else if word_start(&label) || word_start(&english) {
        1
    } else if label.contains(&q) || english.contains(&q) {
        2
    } else {
        3
    })
}

/// The entries matching `query`, best first (menu order among equals). At most `limit`.
pub fn matches(app: &ScriptApp, query: &str, limit: usize) -> Vec<Entry> {
    let mut v: Vec<(u32, usize, Entry)> = entries(app).into_iter().enumerate().filter_map(|(i, e)| score(&e, query).map(|s| (s, i, e))).collect();
    v.sort_by_key(|(s, i, _)| (*s, *i));
    v.into_iter().take(limit).map(|(.., e)| e).collect()
}

/// `ui.commandPalette`: `{"open": bool (default: toggle), "query": string}`. Returns whether it
/// is open and the ids of the best matches for its query.
pub fn command(app: &mut ScriptApp, p: &Value) -> Value {
    let query = p.get("query").and_then(Value::as_str);
    let open = p.get("open").and_then(Value::as_bool).unwrap_or(app.palette.is_none() || query.is_some());
    if open {
        let mut pal = app.palette.take().unwrap_or_default();
        if let Some(q) = query {
            // Typed queries are short; a hostile one is cut, at a character boundary.
            pal.query = q.chars().take(200).collect();
            pal.selected = 0;
            pal.error = None;
        }
        pal.focus = true;
        app.palette = Some(pal);
    } else {
        app.palette = None;
    }
    let q = app.palette.as_ref().map(|p| p.query.clone()).unwrap_or_default();
    let ids: Vec<String> = matches(app, &q, 20).into_iter().map(|e| e.id).collect();
    json!({"open": app.palette.is_some(), "query": q, "matches": ids})
}

/// Run the chosen entry. Closes the palette, unless the command failed (then it says why).
fn run(app: &mut ScriptApp, pal: &mut Palette, e: &Entry) -> bool {
    if e.disabled.is_some() {
        return false;
    }
    match app.execute(&e.id, json!({})) {
        Ok(_) => true,
        Err(err) => {
            pal.error = Some((e.id.clone(), err.to_string()));
            false
        }
    }
}

/// Draw the palette (when open): a floating field over the top of the desk with the matches
/// under it. Up/Down choose, Enter runs, Escape or a click elsewhere closes.
pub fn show(app: &mut ScriptApp, ctx: &egui::Context) {
    let Some(mut pal) = app.palette.take() else { return };
    let lang = app.prefs.appearance.language;
    let t = app.tokens();
    let list = matches(app, &pal.query, usize::MAX);
    let n = list.len();
    // Keys for the list, taken before the text field sees them.
    let (down, up, enter, escape) = ctx.input_mut(|i| {
        (
            i.consume_key(Modifiers::NONE, Key::ArrowDown),
            i.consume_key(Modifiers::NONE, Key::ArrowUp),
            i.consume_key(Modifiers::NONE, Key::Enter),
            i.consume_key(Modifiers::NONE, Key::Escape),
        )
    });
    let moved = down || up;
    if down && n > 0 {
        pal.selected = (pal.selected + 1) % n;
    }
    if up && n > 0 {
        pal.selected = pal.selected.checked_sub(1).unwrap_or(n - 1);
    }
    pal.selected = pal.selected.min(n.saturating_sub(1));
    let mut close = escape;
    let mut chosen: Option<Entry> = if enter { list.get(pal.selected).cloned() } else { None };
    let width = (ctx.content_rect().width() - 2.0 * space::L).clamp(240.0, 600.0);
    let area = egui::Area::new(egui::Id::unique("command-palette"))
        .order(egui::Order::Foreground)
        .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 72.0))
        .show(ctx, |ui| {
            egui::Frame::NONE
                .fill(t.pop)
                .stroke(egui::Stroke::new(1.0, t.chrome_line))
                .corner_radius(radius::L)
                .shadow(egui::Shadow { offset: [0, 8], blur: 28, spread: 0, color: t.shadow })
                .inner_margin(egui::Margin::same(space::M as i8))
                .show(ui, |ui| {
                    ui.set_width(width);
                    ui.horizontal(|ui| {
                        let (icon, _) = ui.allocate_exact_size(egui::vec2(18.0, 18.0), egui::Sense::hover());
                        crate::icons::paint(ui, crate::icons::Icon::Find, icon, t.text_soft);
                        let before = pal.query.clone();
                        let r = ui.add(
                            egui::TextEdit::singleline(&mut pal.query)
                                .id(egui::Id::unique("command-palette-field"))
                                .frame(egui::Frame::NONE)
                                .desired_width(f32::INFINITY)
                                .font(egui::FontId::proportional(type_scale::SUBTITLE))
                                .hint_text(lang.text("Type a command…")),
                        );
                        if pal.focus {
                            r.request_focus();
                            pal.focus = false;
                        }
                        if pal.query != before {
                            pal.selected = 0;
                            pal.error = None;
                        }
                    });
                    ui.add_space(space::S);
                    ui.separator();
                    if let Some((id, msg)) = &pal.error {
                        let need = list.iter().find(|e| &e.id == id).map(|e| e.params.clone()).unwrap_or_default();
                        ui.label(RichText::new(msg).color(t.danger).size(type_scale::SMALL));
                        if need != "{}" && !need.is_empty() {
                            ui.label(
                                RichText::new(lang.format("This command needs details: {params}", &[("params", need)]))
                                    .color(t.text_soft)
                                    .size(type_scale::SMALL),
                            );
                        }
                        ui.add_space(space::XS);
                    }
                    if list.is_empty() {
                        ui.label(RichText::new(lang.text("No command matches")).color(t.text_soft));
                        return;
                    }
                    egui::ScrollArea::vertical().max_height(360.0).auto_shrink([false, true]).show(ui, |ui| {
                        for (i, e) in list.iter().enumerate() {
                            if row(ui, &t, e, i == pal.selected, moved).clicked() {
                                pal.selected = i;
                                chosen = Some(e.clone());
                            }
                        }
                    });
                    ui.add_space(space::XS);
                    ui.label(
                        RichText::new(lang.format("{count} commands · ↑↓ choose · Enter run · Esc close", &[("count", n.to_string())]))
                            .size(type_scale::CAPTION)
                            .color(t.text_soft),
                    );
                });
        });
    if ctx.input(|i| i.pointer.any_pressed()) && !area.response.contains_pointer() && chosen.is_none() {
        close = true;
    }
    if let Some(e) = chosen
        && run(app, &mut pal, &e)
    {
        close = true;
    }
    // The command may have reopened or closed the palette itself; respect that.
    if !close && app.palette.is_none() {
        app.palette = Some(pal);
    } else if close {
        app.palette = None;
        ctx.memory_mut(|m| m.request_focus(crate::canvas::canvas_id()));
    }
}

/// One row: the name, where it lives, and its shortcut.
fn row(ui: &mut Ui, t: &crate::theme::Tokens, e: &Entry, selected: bool, scroll: bool) -> egui::Response {
    let h = 30.0;
    let (rect, r) = ui.allocate_exact_size(egui::vec2(ui.available_width(), h), egui::Sense::click());
    let r = r.on_hover_cursor(egui::CursorIcon::PointingHand);
    if selected && scroll {
        r.scroll_to_me(None);
    }
    let p = ui.painter();
    if selected || r.hovered() {
        p.rect_filled(rect, radius::S, t.accent_weak);
    }
    let text = if e.disabled.is_some() { t.text_soft } else { t.text };
    let x = rect.left() + space::S;
    let cy = rect.center().y;
    let name = p.text(egui::pos2(x, cy), egui::Align2::LEFT_CENTER, &e.label, egui::FontId::proportional(type_scale::BODY), text);
    if !e.location.is_empty() {
        p.text(
            egui::pos2(name.right() + space::S, cy),
            egui::Align2::LEFT_CENTER,
            &e.location,
            egui::FontId::proportional(type_scale::SMALL),
            t.text_soft,
        );
    }
    if !e.shortcut.is_empty() {
        let k = crate::chrome::pretty_key(&e.shortcut);
        let galley = p.layout_no_wrap(k, egui::FontId::proportional(type_scale::CAPTION), t.text_soft);
        let kr = egui::Rect::from_min_size(
            egui::pos2(rect.right() - space::S - galley.size().x - 10.0, cy - galley.size().y / 2.0 - 2.0),
            galley.size() + egui::vec2(10.0, 4.0),
        );
        p.rect_stroke(kr, 4.0, egui::Stroke::new(1.0, t.chrome_line), egui::StrokeKind::Inside);
        p.galley(kr.min + egui::vec2(5.0, 2.0), galley, t.text_soft);
    }
    if let Some(why) = &e.disabled { r.on_hover_text(why.clone()) } else { r }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(id: &str, label: &str, location: &str) -> Entry {
        Entry {
            id: id.into(),
            label: label.into(),
            english: label.into(),
            location: location.into(),
            shortcut: String::new(),
            disabled: None,
            params: "{}".into(),
        }
    }

    #[test]
    fn scores_prefer_names_that_start_with_the_query() {
        let save = entry("ui.save", "Save", "File");
        let save_as = entry("ui.saveAs", "Save As…", "File");
        let auto = entry("x.autosave", "Autosave Now", "File");
        assert_eq!(score(&save, "sa"), Some(0));
        assert_eq!(score(&save_as, "as"), Some(1));
        assert_eq!(score(&auto, "save"), Some(2));
        // Words may come from the menu or the id; every word must be found somewhere.
        assert_eq!(score(&save, "file save"), Some(3));
        assert_eq!(score(&save_as, "ui.saveas"), Some(3));
        assert_eq!(score(&save, "save pdf"), None);
        assert_eq!(score(&save, "   "), Some(3));
        assert_eq!(score(&save, "СОХР"), None);
    }

    #[test]
    fn every_command_is_listed_once_and_file_twins_are_merged() {
        let app = ScriptApp::new(draftcraft_engine::Session::new(Default::default()));
        let all = entries(&app);
        let ids: std::collections::BTreeSet<&str> = all.iter().map(|e| e.id.as_str()).collect();
        assert_eq!(ids.len(), all.len());
        assert!(ids.contains("ui.save") && !ids.contains("file.save"));
        assert!(ids.contains("ui.exportPdf") && !ids.contains("file.exportPdf"));
        assert!(ids.contains("scene.goto") && ids.contains("text.insert"));
        assert!(!ids.contains("ui.commandPalette"));
        assert!(all.len() > 150, "{}", all.len());
    }
}
