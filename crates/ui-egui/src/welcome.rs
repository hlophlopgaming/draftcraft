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

/// What a recent row tells about its file: read from the file once and kept until the file
/// changes (its size or modification time), so drawing never re-reads it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Details {
    /// The file is not there.
    pub missing: bool,
    /// Last change, seconds since the Unix epoch.
    pub modified: Option<u64>,
    /// Pages of script; Fountain and Final Draft files only (imports are not read).
    pub pages: Option<usize>,
    /// The template its formatting matches.
    pub template: Option<Template>,
    /// The latest revision set with marks in its text (`Blue`).
    pub revision: Option<String>,
}

impl Details {
    pub fn to_json(&self, path: &str) -> Value {
        json!({
            "path": path,
            "missing": self.missing,
            "modified": self.modified,
            "pages": self.pages,
            "template": self.template.map(Template::id),
            "revision": self.revision,
        })
    }
}

/// Details read so far, by path, with the (modified, size) stamp they were read at.
pub type DetailsCache = std::collections::HashMap<String, (Option<(u64, u64)>, Details)>;

/// The details of a recent file, from the cache while the file is unchanged. Never on the web,
/// which has no file system (the row then shows its format only).
pub fn details(cache: &mut DetailsCache, path: &str) -> Details {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let p = std::path::Path::new(path);
        let Ok(meta) = std::fs::metadata(p) else {
            cache.remove(path);
            return Details { missing: true, ..Details::default() };
        };
        let modified = meta.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs());
        let stamp = Some((modified.unwrap_or(0), meta.len()));
        if let Some((s, d)) = cache.get(path)
            && *s == stamp
        {
            return d.clone();
        }
        let mut d = Details { modified, ..Details::default() };
        let native =
            matches!(draftcraft_engine::io::Format::of(p), Some(draftcraft_engine::io::Format::Fountain | draftcraft_engine::io::Format::Fdx));
        if native
            && meta.is_file()
            && let Ok(info) = draftcraft_engine::io::info(p)
        {
            d.pages = Some(info.pages);
            d.template = info.template;
            d.revision = info.revision;
        }
        // The list holds at most `persist::MAX_RECENT` files; drop paths no longer in it.
        if cache.len() > 4 * crate::persist::MAX_RECENT {
            cache.clear();
        }
        cache.insert(path.to_string(), (stamp, d.clone()));
        d
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (cache, path);
        Details::default()
    }
}

/// A local date and time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Civil {
    pub year: i32,
    /// 1–12.
    pub month: u8,
    /// 1–31.
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

impl Civil {
    /// Days since 1970-01-01 (proleptic Gregorian), to tell "yesterday" across months and years.
    fn day_number(self) -> i64 {
        let (m, d) = (i64::from(self.month.clamp(1, 12)), i64::from(self.day.clamp(1, 31)));
        let y = i64::from(self.year) - i64::from(m <= 2);
        let era = y.div_euclid(400);
        let yoe = y - era * 400;
        let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146_097 + doe - 719_468
    }
}

/// Unix seconds as local time (the system's time zone). `None` on the web and for times out of
/// range.
pub fn local(secs: u64) -> Option<Civil> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let ts = jiff::Timestamp::from_second(i64::try_from(secs).ok()?).ok()?;
        let z = ts.to_zoned(jiff::tz::TimeZone::system());
        Some(Civil {
            year: i32::from(z.year()),
            month: u8::try_from(z.month()).ok()?,
            day: u8::try_from(z.day()).ok()?,
            hour: u8::try_from(z.hour()).ok()?,
            minute: u8::try_from(z.minute()).ok()?,
        })
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = secs;
        None
    }
}

/// Now as local time; `None` on the web.
pub fn now() -> Option<Civil> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).ok()?.as_secs();
        local(secs)
    }
    #[cfg(target_arch = "wasm32")]
    {
        None
    }
}

/// Month abbreviations, catalogue keys.
const MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

/// When a file was changed, as the row shows it: `Today, 11:40`, `Yesterday`, `12 Nov`, or
/// `2 Nov 2025` for another year.
pub fn when(lang: crate::i18n::Language, t: Civil, today: Civil) -> String {
    let days = today.day_number() - t.day_number();
    let time = format!("{:02}:{:02}", t.hour.min(23), t.minute.min(59));
    if days == 0 {
        return lang.format("Today, {time}", &[("time", time)]);
    }
    if days == 1 {
        return lang.text("Yesterday").to_string();
    }
    let month = MONTHS.get(usize::from(t.month.clamp(1, 12)) - 1).map_or("", |m| lang.text(m));
    if t.year == today.year { format!("{} {month}", t.day) } else { format!("{} {month} {}", t.day, t.year) }
}

/// The greeting for the hour: morning from 5, afternoon from 12, evening from 18, night from 23.
pub fn greeting(hour: u8) -> &'static str {
    match hour {
        5..=11 => "Good morning.",
        12..=17 => "Good afternoon.",
        18..=22 => "Good evening.",
        _ => "Good night.",
    }
}

/// The kind of script a row shows: the "New" card's name for its template, else its format.
fn kind(lang: crate::i18n::Language, f: &Recent, d: &Details) -> String {
    let base = d
        .template
        .and_then(|t| NEW_CARDS.iter().find(|(c, ..)| *c == t).map(|(_, label, _)| *label))
        .map_or_else(|| lang.text(f.format).to_string(), |l| lang.text(l).to_string());
    match d.pages {
        Some(1) => format!("{base} · {}", lang.text("1 page")),
        Some(n) => format!("{base} · {}", lang.format("{pages} pages", &[("pages", n.to_string())])),
        None => base,
    }
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
/// open, whether it shows at start, the recent files (`recent`: paths; `files`: each with its
/// pages, template, latest revision and modification time) and the templates it offers.
pub fn command(app: &mut ScriptApp, p: &Value) -> Value {
    app.welcome = p.get("open").and_then(Value::as_bool).unwrap_or(!app.welcome);
    if let Some(on) = p.get("atStart").and_then(Value::as_bool) {
        app.prefs.appearance.show_welcome = on;
    }
    let recent: Vec<String> = recent(app).into_iter().map(|r| r.path).collect();
    let files: Vec<Value> = recent.iter().map(|p| details(&mut app.recent_details, p).to_json(p)).collect();
    let templates: Vec<&str> = NEW_CARDS.iter().map(|(t, ..)| t.id()).collect();
    json!({"open": app.welcome, "atStart": app.prefs.appearance.show_welcome, "recent": recent, "files": files, "templates": templates})
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

/// The greeting for the time of day, what is open, and "Continue writing".
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
            // The greeting for the time of day, as in the mockup; the web has no local clock.
            let title = now().map_or("Welcome to DraftCraft", |c| greeting(c.hour));
            ui.label(RichText::new(lang.text(title)).size(type_scale::DISPLAY).strong().color(t.text))
                .on_hover_text(lang.text("Welcome to DraftCraft"));
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
        let today = now();
        for (i, f) in list.into_iter().enumerate() {
            let d = details(&mut app.recent_details, &f.path);
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
            let one_line = |text: String, font: egui::FontId, color: Color32, w: f32| {
                let mut job = egui::text::LayoutJob::simple_singleline(text, font, color);
                job.wrap = egui::text::TextWrapping::truncate_at_width(w.max(1.0));
                p.layout_job(job)
            };
            // Columns from the right, as in the mockup: the revision badge, the date, then the
            // kind and pages. Narrow windows drop the date, then the kind.
            let right = r.right() - space::L;
            let badge_w = 96.0;
            let date_w = 120.0;
            let kind_w = 190.0;
            let room = r.width() - 2.0 * space::L - 28.0;
            let show_date = room > 360.0 + badge_w + date_w;
            let show_kind = room > 220.0 + badge_w;
            if let Some(rev) = &d.revision {
                let label = lang.format("{name} rev.", &[("name", lang.text(rev).to_string())]);
                let g = one_line(label, small.clone(), t.text, badge_w - 2.0 * space::S);
                let pill = Rect::from_min_size(
                    pos2(right - g.size().x - 2.0 * space::S, r.center().y - g.size().y / 2.0 - 3.0),
                    vec2(g.size().x + 2.0 * space::S, g.size().y + 6.0),
                );
                p.rect_filled(pill, CornerRadius::from(radius::M), t.chrome_line);
                p.galley(pos2(pill.left() + space::S, pill.top() + 3.0), g, t.text);
            }
            let mut col = right - badge_w - space::M;
            if show_date && let (Some(m), Some(today)) = (d.modified.and_then(local), today) {
                let g = one_line(when(lang, m, today), small.clone(), t.text_soft, date_w);
                p.galley(pos2(col - date_w, r.center().y - g.size().y / 2.0), g, t.text_soft);
            }
            if show_date {
                col -= date_w + space::M;
            }
            let kind_text = if d.missing { lang.text("Not found").to_string() } else { kind(lang, &f, &d) };
            if show_kind {
                let g = one_line(kind_text, small.clone(), t.text_soft, kind_w);
                p.galley(pos2(col - kind_w, r.center().y - g.size().y / 2.0), g, t.text_soft);
                col -= kind_w + space::M;
            }
            // The name, and the folder after it in the space that is left.
            let name_x = x + 28.0;
            let name_room = (col - name_x).max(40.0);
            let name = one_line(f.name.clone(), body, if d.missing { t.text_soft } else { t.text }, name_room);
            let name_w = name.size().x;
            p.galley(pos2(name_x, r.center().y - name.size().y / 2.0), name, t.text);
            let folder_x = name_x + name_w + space::M;
            if col - folder_x > 60.0 {
                let g = one_line(f.folder.clone(), small, t.text_soft, col - folder_x);
                p.galley(pos2(folder_x, r.center().y - g.size().y / 2.0), g, t.text_soft);
            }
            let resp = resp.on_hover_text(format!("{}\n{}", f.path, lang.text(f.format))).on_hover_cursor(egui::CursorIcon::PointingHand);
            if resp.clicked() {
                *run = Some(("ui.openRecent".into(), json!({"path": f.path})));
            }
        }
    });
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Language;

    fn at(year: i32, month: u8, day: u8, hour: u8, minute: u8) -> Civil {
        Civil { year, month, day, hour, minute }
    }

    #[test]
    fn dates_read_like_the_mockup() {
        let today = at(2026, 11, 26, 18, 5);
        assert_eq!(when(Language::English, at(2026, 11, 26, 9, 7), today), "Today, 09:07");
        assert_eq!(when(Language::English, at(2026, 11, 25, 23, 59), today), "Yesterday");
        assert_eq!(when(Language::English, at(2026, 11, 12, 10, 0), today), "12 Nov");
        assert_eq!(when(Language::English, at(2025, 11, 2, 10, 0), today), "2 Nov 2025");
        assert_eq!(when(Language::Russian, at(2026, 11, 26, 11, 40), today), "Сегодня, 11:40");
        assert_eq!(when(Language::Russian, at(2026, 5, 9, 11, 40), today), "9 мая");
        // Yesterday across a month and a year, and in a leap year.
        assert_eq!(when(Language::English, at(2026, 10, 31, 1, 0), at(2026, 11, 1, 1, 0)), "Yesterday");
        assert_eq!(when(Language::English, at(2025, 12, 31, 1, 0), at(2026, 1, 1, 1, 0)), "Yesterday");
        assert_eq!(when(Language::English, at(2028, 2, 29, 1, 0), at(2028, 3, 1, 1, 0)), "Yesterday");
        assert_eq!(at(1970, 1, 1, 0, 0).day_number(), 0);
        // Hostile values never panic.
        for c in [at(i32::MIN, 0, 0, 255, 255), at(i32::MAX, 13, 99, 24, 60)] {
            let _ = when(Language::English, c, today);
        }
    }

    #[test]
    fn greetings_follow_the_hour() {
        assert_eq!(greeting(4), "Good night.");
        assert_eq!(greeting(5), "Good morning.");
        assert_eq!(greeting(12), "Good afternoon.");
        assert_eq!(greeting(18), "Good evening.");
        assert_eq!(greeting(23), "Good night.");
        assert_eq!(greeting(200), "Good night.");
    }

    #[test]
    fn local_time_is_read() {
        assert!(now().is_some());
        let c = local(0).unwrap();
        assert!(c.year == 1970 || c.year == 1969);
        assert!(local(u64::MAX).is_none());
    }
}
