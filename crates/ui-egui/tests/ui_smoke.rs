//! The window runs headless (no GPU): typing through egui reaches the engine, every UI command
//! runs, and all views lay out without panicking.

use draftcraft_engine::Session;
use draftcraft_ui_egui::{ControlRequest, ScriptApp, UI_COMMANDS};
use egui_kittest::kittest::Queryable;
use serde_json::json;

fn harness(app: ScriptApp) -> egui_kittest::Harness<'static, ScriptApp> {
    egui_kittest::Harness::builder().with_size(egui::vec2(1280.0, 800.0)).build_ui_state(
        |ui, app: &mut ScriptApp| {
            let ctx = ui.ctx().clone();
            app.logic(&ctx);
            app.ui(ui);
        },
        app,
    )
}

fn step(h: &mut egui_kittest::Harness<'static, ScriptApp>, n: usize) {
    for _ in 0..n {
        let mut raw = std::mem::take(h.input_mut());
        h.state_mut().raw_input_hook(&mut raw);
        *h.input_mut() = raw;
        h.step();
    }
}

#[test]
fn typing_through_the_window() {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut app = ScriptApp::new(Session::new(Default::default())).with_control(rx);
    app.dialogs = false;
    let mut h = harness(app);
    step(&mut h, 4);
    let send = |m: &str, p: serde_json::Value| {
        let (req, reply) = ControlRequest::new(m, p);
        tx.send(req).unwrap();
        reply
    };
    let mut replies = Vec::new();
    for (m, p) in [
        ("ui.text", json!({"text": "int. lab - night"})),
        ("ui.key", json!({"key": "Enter"})),
        ("ui.text", json!({"text": "Beakers bubble."})),
        ("ui.key", json!({"key": "Tab"})),
        ("ui.text", json!({"text": "ada"})),
        ("ui.key", json!({"key": "Enter"})),
        ("ui.text", json!({"text": "Eureka."})),
    ] {
        replies.push(send(m, p));
        step(&mut h, 3);
    }
    for r in replies {
        assert_eq!(r.try_recv().unwrap()["ok"], true);
    }
    let s = &h.state().session.script;
    let got: Vec<(&str, &str)> = s.paras.iter().map(|p| (p.element.id(), p.text.as_str())).collect();
    assert_eq!(got, vec![("sceneHeading", "INT. LAB - NIGHT"), ("action", "Beakers bubble."), ("character", "ADA"), ("dialogue", "Eureka.")]);
    let r = send("ui.inspect", json!({}));
    step(&mut h, 2);
    assert_eq!(r.try_recv().unwrap()["result"]["element"], "dialogue");
}

#[test]
fn every_ui_command_and_view_runs() {
    let mut app = ScriptApp::new(Session::sample());
    app.dialogs = false;
    let mut h = harness(app);
    step(&mut h, 3);
    for (id, ..) in UI_COMMANDS {
        let _ = h.state_mut().execute(id, json!({}));
        step(&mut h, 2);
    }
    for v in ["ui.view.cards", "ui.view.reports", "ui.theme.toggle", "ui.view.page", "ui.zoomIn", "ui.toggleNavigator"] {
        h.state_mut().execute(v, json!({})).unwrap();
        step(&mut h, 2);
    }
    assert!(h.state_mut().execute("ui.nope", json!({})).is_err());
    let cmds = h.state().describe_commands();
    assert!(cmds.as_array().unwrap().iter().any(|c| c["id"] == "ui.view.cards"));
}

#[test]
fn enter_in_an_empty_paragraph_shows_the_element_list() {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut app = ScriptApp::new(Session::new(Default::default())).with_control(rx);
    app.dialogs = false;
    let mut h = harness(app);
    step(&mut h, 4);
    let send = |m: &str, p: serde_json::Value| {
        let (req, _reply) = ControlRequest::new(m, p);
        tx.send(req).unwrap();
    };
    send("ui.text", json!({"text": "int. lab - night"}));
    step(&mut h, 3);
    send("ui.key", json!({"key": "Enter"}));
    step(&mut h, 3);
    // The new paragraph is an empty Action: Enter opens the list instead of changing it.
    send("ui.key", json!({"key": "Enter"}));
    step(&mut h, 3);
    assert!(h.state().canvas.picker.is_some());
    assert_eq!(h.state().session.element(), draftcraft_doc::Element::Action);
    send("ui.key", json!({"key": "ArrowDown"}));
    send("ui.key", json!({"key": "Enter"}));
    step(&mut h, 4);
    assert!(h.state().canvas.picker.is_none());
    // The list opened on Scene Heading (Enter's usual target); one row down is Action.
    assert_eq!(h.state().session.element(), draftcraft_doc::Element::Action);
    send("ui.key", json!({"key": "Enter"}));
    step(&mut h, 3);
    send("ui.text", json!({"text": "3"}));
    step(&mut h, 3);
    assert_eq!(h.state().session.element(), draftcraft_doc::Element::Character);
    // Turned off, Enter changes the element directly.
    h.state_mut().enter_picker = false;
    send("ui.key", json!({"key": "Enter"}));
    step(&mut h, 3);
    assert_eq!(h.state().session.element(), draftcraft_doc::Element::Action);
    assert_eq!(h.state().session.script.paras.len(), 2);
}

#[test]
fn autosave_backups_and_recent_files() {
    use draftcraft_ui_egui::persist;
    let dir = std::env::temp_dir().join(format!("draftcraft-ui-state-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("draft.fountain");
    std::fs::write(&file, draftcraft_engine::sample::FOUNTAIN).unwrap();
    let mut app = ScriptApp::new(Session::new(Default::default())).with_state_dir(dir.clone());
    app.dialogs = false;
    app.prefs.files.autosave_seconds = 1;
    let mut h = harness(app);
    step(&mut h, 3);
    h.state_mut().execute("file.open", json!({"path": file.to_str().unwrap()})).unwrap();
    assert_eq!(h.state().prefs.files.recent, vec![file.display().to_string()]);
    h.state_mut().execute("text.insert", json!({"text": "Edited. "})).unwrap();
    assert!(h.state().session.dirty);
    step(&mut h, 12);
    let r = persist::read_recovery(&dir).expect("autosaved");
    assert!(r.script.paras[0].text.starts_with("Edited."));
    // Saving keeps the previous version and removes the recovery copy.
    h.state_mut().execute("ui.save", json!({})).unwrap();
    assert!(!h.state().session.dirty);
    assert!(persist::read_recovery(&dir).is_none());
    let backups: Vec<_> = std::fs::read_dir(dir.join(persist::BACKUPS)).unwrap().flatten().collect();
    assert_eq!(backups.len(), 1);
    assert_eq!(std::fs::read_to_string(backups[0].path()).unwrap(), draftcraft_engine::sample::FOUNTAIN);
    // Settings are written to config.toml when the app exits.
    h.state_mut().on_exit();
    let toml = std::fs::read_to_string(dir.join("config.toml")).unwrap();
    assert!(toml.contains("[files]") && toml.contains("draft.fountain"), "{toml}");
    // A recovery file left by a crash is offered on the next start.
    persist::write_recovery(&dir, Some(&file), &draftcraft_engine::sample::script()).unwrap();
    let next = ScriptApp::new(Session::new(Default::default())).with_state_dir(dir.clone());
    assert!(next.recovery_offer.is_some());
    assert_eq!(next.prefs.files.recent.len(), 1);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn new_script_with_unsaved_changes_asks_first() {
    let mut app = ScriptApp::new(Session::sample());
    app.execute("text.insert", json!({"text": "x"})).unwrap();
    let before = app.session.script.paras.len();
    let r = app.execute("file.new", json!({})).unwrap();
    assert_eq!(r["pending"], "unsaved changes");
    assert_eq!(app.session.script.paras.len(), before, "nothing replaced yet");
    assert!(app.confirm_replace.is_some());
    // Agents and tests (no dialogs) are never asked.
    let mut quiet = ScriptApp::new(Session::sample());
    quiet.dialogs = false;
    quiet.execute("text.insert", json!({"text": "x"})).unwrap();
    quiet.execute("file.new", json!({})).unwrap();
    assert_eq!(quiet.session.script.paras.len(), 1);
}

#[test]
fn speed_view_and_zoom() {
    let mut app = ScriptApp::new(Session::sample());
    app.dialogs = false;
    app.canvas.blink = false;
    let mut h = harness(app);
    step(&mut h, 3);
    h.state_mut().execute("ui.view.speed", json!({})).unwrap();
    step(&mut h, 2);
    assert!(h.state().session.speed);
    assert_eq!(h.state().canvas.page_rects.len(), 1, "one continuous page");
    // Reports and the status bar keep real pages.
    assert_eq!(h.state_mut().session.layout().script_pages(), 4);
    h.state_mut().execute("caret.docEnd", json!({})).unwrap();
    h.state_mut().execute("caret.up", json!({})).unwrap();
    h.state_mut().execute("ui.zoomFit", json!({})).unwrap();
    step(&mut h, 2);
    let z = h.state().canvas.zoom;
    assert!((0.4..=3.0).contains(&z) && z > 1.0, "{z}");
    h.state_mut().execute("ui.view.page", json!({})).unwrap();
    step(&mut h, 2);
    assert!(!h.state().session.speed);
    assert!(h.state().canvas.page_rects.len() > 1);
}

#[test]
fn closing_with_unsaved_changes_asks() {
    let mut app = ScriptApp::new(Session::sample());
    app.dialogs = false;
    // Clean: closes at once.
    assert!(app.close_requested());
    // Quit goes through the same path as the close button.
    app.execute("ui.quit", json!({})).unwrap();
    assert!(app.close_wanted);
    app.execute("text.insert", json!({"text": "x"})).unwrap();
    assert!(!app.close_requested(), "dirty: ask first");
    assert!(app.confirm_close);
    // "Don't Save" sets quit_requested; the next close goes through.
    app.confirm_close = false;
    app.quit_requested = true;
    assert!(app.close_requested());
    let mut h = harness(ScriptApp::new(Session::sample()));
    step(&mut h, 3);
    h.state_mut().execute("text.insert", json!({"text": "x"})).unwrap();
    assert!(!h.state_mut().close_requested());
    step(&mut h, 3);
    let names: Vec<String> = h.state().dialog_buttons.iter().map(|(n, _)| n.clone()).collect();
    assert_eq!(names, ["Save", "Don't Save", "Cancel"]);
}

/// A file host like the browser's: records downloads, and "picks" a file into the inbox.
struct MockHost {
    saved: std::sync::Arc<std::sync::Mutex<Vec<(String, Vec<u8>)>>>,
    pick: Option<(String, Vec<u8>)>,
}

impl draftcraft_ui_egui::FileHost for MockHost {
    fn open(&self, inbox: draftcraft_ui_egui::Inbox) {
        if let Some(f) = self.pick.clone() {
            inbox.lock().unwrap().push(f);
        }
    }
    fn save(&self, name: &str, bytes: Vec<u8>) {
        self.saved.lock().unwrap().push((name.to_string(), bytes));
    }
}

#[test]
fn browser_style_open_and_download() {
    let saved = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let mut app = ScriptApp::new(Session::sample());
    app.dialogs = false;
    app.host = Some(Box::new(MockHost { saved: saved.clone(), pick: Some(("pier.fountain".into(), b"EXT. PIER - DAWN\n\nGulls.\n".to_vec())) }));
    let mut h = harness(app);
    step(&mut h, 3);
    h.state_mut().execute("ui.exportPdf", json!({})).unwrap();
    h.state_mut().execute("ui.exportFdx", json!({})).unwrap();
    {
        let s = saved.lock().unwrap();
        assert_eq!(s[0].0, "Night Signal.pdf");
        assert!(s[0].1.starts_with(b"%PDF"));
        assert_eq!(s[1].0, "Night Signal.fdx");
    }
    h.state_mut().execute("ui.open", json!({})).unwrap();
    step(&mut h, 2);
    let st = h.state();
    assert_eq!(st.session.script.paras.len(), 2);
    assert_eq!(st.session.script.paras[0].text, "EXT. PIER - DAWN");
}

#[test]
fn misspellings_update_as_you_type() {
    // Regression: the spelling cache was keyed by the layout's address, which a new layout can
    // reuse, so a word typed later was never underlined.
    let mut app = ScriptApp::new(Session::sample());
    app.dialogs = false;
    let mut h = harness(app);
    step(&mut h, 3);
    for t in ["The ", "nedle ", "shivers."] {
        h.state_mut().execute("caret.docEnd", json!({})).unwrap();
        h.state_mut().execute("text.insert", json!({"text": t})).unwrap();
        step(&mut h, 2);
    }
    let st = h.state();
    let words: Vec<&str> = st.canvas.underlined().iter().filter_map(|&(p, a, b)| st.session.script.para(p)?.text.get(a..b)).collect();
    assert!(words.contains(&"NEDLE"), "underlined: {words:?}");
    let rev = h.state().session.revision();
    h.state_mut().execute("text.insert", json!({"text": "x"})).unwrap();
    assert!(h.state().session.revision() > rev);
}

#[test]
fn outline_view_drags_a_scene_under_an_act() {
    let mut app = ScriptApp::new(Session::sample());
    app.dialogs = false;
    let mut h = harness(app);
    step(&mut h, 3);
    h.state_mut().execute("structure.add", json!({"title": "One", "before": 0})).unwrap();
    h.state_mut().execute("structure.add", json!({"title": "Two", "before": 2})).unwrap();
    h.state_mut().execute("ui.view.outline", json!({})).unwrap();
    h.state_mut().execute("ui.toggleStoryMap", json!({})).unwrap();
    step(&mut h, 4);
    let rows: Vec<egui::Rect> = h.ctx.data(|d| d.get_temp(draftcraft_ui_egui::outline::rows_id())).unwrap_or_default();
    assert_eq!(rows.len(), 6, "[One] 0 1 [Two] 2 3");
    let last_heading = h.state().session.script.scenes()[3].heading.clone();
    // Grab the last scene's grip (it is under act two: one level in) and drop it just below [One].
    let grip = egui::pos2(rows[5].left() + 26.0 + 11.0, rows[5].center().y);
    let drop = egui::pos2(grip.x, rows[1].top() + 4.0);
    let ev = |e: egui::Event, h: &mut egui_kittest::Harness<'static, ScriptApp>| h.state_mut().synthetic.push(e);
    ev(egui::Event::PointerMoved(grip), &mut h);
    step(&mut h, 1);
    ev(egui::Event::PointerButton { pos: grip, button: egui::PointerButton::Primary, pressed: true, modifiers: Default::default() }, &mut h);
    step(&mut h, 1);
    for k in 1..=8 {
        ev(egui::Event::PointerMoved(grip + (drop - grip) * (k as f32 / 8.0)), &mut h);
        step(&mut h, 1);
    }
    ev(egui::Event::PointerButton { pos: drop, button: egui::PointerButton::Primary, pressed: false, modifiers: Default::default() }, &mut h);
    step(&mut h, 3);
    let s = &h.state().session.script;
    assert_eq!(s.scenes()[0].heading, last_heading);
    // Act one opens the moved scene; act two still opens scene 2's old neighbour.
    assert_eq!(s.paras[s.scenes()[0].para].structure.first().map(|p| p.title.as_str()), Some("One"));
    assert_eq!(s.structure_count(), 2);
    // The story map and navigator draw with structure.
    h.state_mut().execute("ui.view.page", json!({})).unwrap();
    step(&mut h, 2);
}

#[test]
fn import_from_word_celtx_and_pdf() {
    // Through the host (the browser): the picked file arrives in the inbox and is imported.
    let saved = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let mut app = ScriptApp::new(Session::sample());
    app.dialogs = false;
    let html = br#"<p class="sceneheading">INT. LAMP ROOM - NIGHT</p><p class="character">NELL</p><p class="dialog">Light it.</p>"#;
    app.host = Some(Box::new(MockHost { saved, pick: Some(("lamp.celtx".into(), html.to_vec())) }));
    let mut h = harness(app);
    step(&mut h, 2);
    h.state_mut().execute("ui.import", json!({})).unwrap();
    step(&mut h, 2);
    let st = h.state();
    assert_eq!(st.session.script.paras.len(), 3);
    assert_eq!(st.session.script.paras[2].text, "Light it.");
    assert!(st.session.dirty && st.session.path.is_none());
    // Natively, with a path: a PDF exported by DraftCraft comes back as the same script.
    let dir = std::env::temp_dir().join(format!("sc-ui-import-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let pdf = dir.join("night.pdf");
    let mut s = Session::sample();
    s.run("file.exportPdf", &json!({"path": pdf})).unwrap();
    let want = s.script.paras.len();
    let mut app = ScriptApp::new(Session::new(Default::default()));
    app.dialogs = false;
    let mut h = harness(app);
    step(&mut h, 2);
    let v = h.state_mut().execute("ui.import", json!({"path": pdf})).unwrap();
    assert_eq!(v["format"], "PDF");
    assert_eq!(h.state().session.script.paras.len(), want);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn russian_language_switch_preserves_script_and_command_ids() {
    let mut app = ScriptApp::new(Session::new(Default::default()));
    app.dialogs = false;
    let original = app.session.script.clone();
    app.execute("ui.language", json!({"language": "ru"})).unwrap();
    assert_eq!(app.prefs.appearance.language, draftcraft_ui_egui::i18n::Language::Russian);
    assert_eq!(app.session.script, original);
    assert!(app.execute("ui.language", json!({"language": "unknown"})).is_err());
    let mut h = harness(app);
    step(&mut h, 4);
    assert!(h.query_by_label("Файл").is_some());
    assert!(h.query_by_label("File").is_none());
    for (id, params) in [
        ("text.insert", json!({"text": "ИНТ. КУХНЯ — НОЧЬ"})),
        ("text.enter", json!({})),
        ("text.insert", json!({"text": "Ёжик ждёт у окна."})),
        ("ui.view.cards", json!({})),
        ("ui.view.reports", json!({})),
        ("ui.titlePage", json!({})),
        ("ui.elementSettings", json!({})),
    ] {
        h.state_mut().execute(id, params).unwrap();
        step(&mut h, 2);
    }
    assert!(h.state().session.script.paras.iter().any(|p| p.text == "Ёжик ждёт у окна."));
    h.state_mut().execute("ui.language", json!({"language": "en"})).unwrap();
    assert_eq!(h.state().prefs.appearance.language, draftcraft_ui_egui::i18n::Language::English);
    step(&mut h, 2);
    assert!(h.query_by_label("File").is_some());
    assert!(h.query_by_label("Файл").is_none());
}

#[test]
fn panels_dock_to_any_side() {
    let mut app = ScriptApp::new(Session::sample());
    app.dialogs = false;
    let mut h = harness(app);
    step(&mut h, 3);
    let rect_of = |h: &egui_kittest::Harness<'static, ScriptApp>, label: &str| h.get_by_label(label).rect();
    // By default the navigator is on the left.
    assert!(rect_of(&h, "NAVIGATOR").center().x < 400.0);
    // Docking it to the right moves it there and is reported in the state.
    let v = h.state_mut().execute("ui.dockPanel", json!({"panel": "navigator", "side": "right"})).unwrap();
    assert_eq!(v["layout"]["navigator"], "right");
    step(&mut h, 3);
    assert!(rect_of(&h, "NAVIGATOR").center().x > 880.0);
    // Docking a hidden story map shows it, here at the top, under the toolbar.
    assert!(!h.state().story_map);
    let v = h.state_mut().execute("ui.dockPanel", json!({"panel": "storyMap", "side": "top"})).unwrap();
    assert_eq!((v["storyMap"].as_bool(), v["layout"]["storyMap"].as_str()), (Some(true), Some("top")));
    step(&mut h, 3);
    assert!(rect_of(&h, "STORY MAP").center().y < 250.0);
    // Then to the left: a column on the left edge.
    h.state_mut().execute("ui.dockPanel", json!({"panel": "map", "side": "left"})).unwrap();
    step(&mut h, 3);
    let map = rect_of(&h, "STORY MAP");
    assert!(map.center().x < 400.0 && map.center().y < 250.0, "{map:?}");
    // The settings keep the choice.
    assert_eq!(h.state().prefs.layout.story_map, draftcraft_ui_egui::dock::Side::Left);
    // Bad or missing parameters are errors, never a crash, and change nothing.
    for p in [
        json!({}),
        json!({"panel": "navigator"}),
        json!({"panel": "toolbar", "side": "left"}),
        json!({"panel": "navigator", "side": "diagonal"}),
        json!({"panel": 7, "side": []}),
    ] {
        assert!(h.state_mut().execute("ui.dockPanel", p).is_err());
    }
    assert_eq!(h.state().prefs.layout.navigator, draftcraft_ui_egui::dock::Side::Right);
    step(&mut h, 2);
}

#[test]
fn command_palette_finds_and_runs_commands() {
    let mut app = ScriptApp::new(Session::sample());
    app.dialogs = false;
    let mut h = harness(app);
    step(&mut h, 3);
    let key = |h: &mut egui_kittest::Harness<'static, ScriptApp>, key: egui::Key| {
        for pressed in [true, false] {
            h.input_mut().events.push(egui::Event::Key { key, physical_key: None, pressed, repeat: false, modifiers: egui::Modifiers::NONE });
        }
        step(h, 2);
    };
    // Mod+K opens it with the field focused; typing filters the list.
    h.input_mut().events.push(egui::Event::Key {
        key: egui::Key::K,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::COMMAND,
    });
    step(&mut h, 3);
    assert!(h.state().palette.is_some());
    h.input_mut().events.push(egui::Event::Text("index cards".into()));
    step(&mut h, 2);
    assert_eq!(h.state().palette.as_ref().map(|p| p.query.as_str()), Some("index cards"));
    // Enter runs the best match and closes the palette.
    key(&mut h, egui::Key::Enter);
    assert_eq!(h.state().view, draftcraft_ui_egui::View::Cards);
    assert!(h.state().palette.is_none());
    // The command reports its matches, best first: a name that starts with the query wins.
    let v = h.state_mut().execute("ui.commandPalette", json!({"query": "dark"})).unwrap();
    assert_eq!((v["open"].as_bool(), v["matches"][0].as_str()), (Some(true), Some("ui.theme.toggle")));
    // Arrow keys choose, Enter runs the chosen one.
    let v = h.state_mut().execute("ui.commandPalette", json!({"query": "page view"})).unwrap();
    assert_eq!(v["matches"][0], "ui.view.page");
    step(&mut h, 2);
    key(&mut h, egui::Key::Enter);
    assert_eq!(h.state().view, draftcraft_ui_egui::View::Page);
    // A command that needs details keeps the palette open and says what it needs.
    let v = h.state_mut().execute("ui.commandPalette", json!({"query": "scene.move"})).unwrap();
    assert_eq!(v["matches"][0], "scene.move");
    step(&mut h, 2);
    let before = h.state().session.script.clone();
    key(&mut h, egui::Key::Enter);
    let pal = h.state().palette.clone().expect("palette stays open");
    assert_eq!(pal.error.map(|(id, _)| id).as_deref(), Some("scene.move"));
    assert_eq!(h.state().session.script, before);
    // Escape closes it; nothing matches nonsense; the window survives hostile queries.
    key(&mut h, egui::Key::Escape);
    assert!(h.state().palette.is_none());
    let v = h.state_mut().execute("ui.commandPalette", json!({"query": "zzqqxx"})).unwrap();
    assert_eq!(v["matches"].as_array().map(Vec::len), Some(0));
    step(&mut h, 2);
    for p in [json!({"query": "ё".repeat(10_000)}), json!({"query": 7}), json!({"open": "yes"}), json!({"open": false})] {
        assert!(h.state_mut().execute("ui.commandPalette", p).is_ok());
        step(&mut h, 2);
    }
    assert!(h.state().palette.is_none());
    // In Russian the names are translated and found in either language.
    h.state_mut().execute("ui.language", json!({"language": "ru"})).unwrap();
    for q in ["карточки", "index cards"] {
        let v = h.state_mut().execute("ui.commandPalette", json!({"query": q})).unwrap();
        assert_eq!(v["matches"][0], "ui.view.cards", "{q}");
    }
    step(&mut h, 2);
    assert!(h.query_by_label("Введите команду…").is_some() || h.query_by_label("Карточки сцен").is_some());
}

#[test]
fn welcome_screen_starts_scripts_and_opens_recent_files() {
    let dir = std::env::temp_dir().join(format!("dc-welcome-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let file = dir.join("Gull Point.fountain");
    std::fs::write(&file, "INT. LIGHTHOUSE - NIGHT\n\nThe lamp turns.\n").unwrap();
    let mut app = ScriptApp::new(Session::sample());
    app.dialogs = false;
    app.prefs.touch_recent("/nowhere/Lost.fdx");
    app.prefs.touch_recent(&file.display().to_string());
    // On at start by default; not when a file or a choice was given on the command line.
    assert!(draftcraft_ui_egui::welcome::at_start(&app, false));
    assert!(!draftcraft_ui_egui::welcome::at_start(&app, true));
    let v = app.execute("ui.welcome", json!({"open": true})).unwrap();
    assert_eq!(v["open"], true);
    assert_eq!(v["recent"][0], file.display().to_string());
    assert_eq!(v["templates"].as_array().unwrap().len(), 4);
    let mut h = harness(app);
    step(&mut h, 4);
    assert!(h.query_by_label("Welcome to DraftCraft").is_some());
    assert!(h.query_by_label("Lost.fdx").is_some());
    // A card starts a script from its template and the window returns to the page.
    h.get_by_label("Stage play").click();
    step(&mut h, 3);
    assert!(!h.state().welcome);
    assert_eq!(h.state_mut().execute("template.current", json!({})).unwrap()["id"], "stage-play");
    assert!(h.query_by_label("Welcome to DraftCraft").is_none());
    // A recent file opens with a click.
    h.state_mut().execute("ui.welcome", json!({})).unwrap();
    step(&mut h, 3);
    h.get_by_label("Gull Point.fountain").click();
    step(&mut h, 3);
    assert!(!h.state().welcome);
    assert_eq!(h.state().session.path.as_deref(), Some(file.as_path()));
    // Escape closes it; "Show at start" is a setting.
    h.state_mut().execute("ui.welcome", json!({"open": true, "atStart": false})).unwrap();
    step(&mut h, 2);
    assert!(!h.state().prefs.appearance.show_welcome);
    assert!(!draftcraft_ui_egui::welcome::at_start(h.state(), false));
    h.key_press(egui::Key::Escape);
    step(&mut h, 2);
    assert!(!h.state().welcome);
    // Junk parameters never break it.
    for p in [json!({"open": "yes"}), json!({"atStart": 3}), json!(null), json!([1, 2])] {
        assert!(h.state_mut().execute("ui.welcome", p).is_ok());
        step(&mut h, 1);
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn interface_size_scales_the_window_but_not_the_page() {
    let mut app = ScriptApp::new(Session::sample());
    app.dialogs = false;
    app.welcome = false;
    let mut h = harness(app);
    step(&mut h, 3);
    assert_eq!(h.state().prefs.appearance.interface_size, 100);
    assert!((h.ctx.zoom_factor() - 1.0).abs() < 1e-3);
    let page_before = h.state().page_zoom() * h.ctx.zoom_factor();

    h.state_mut().execute("ui.interfaceSize", json!({"percent": 150})).unwrap();
    step(&mut h, 2);
    assert_eq!(h.state().prefs.appearance.interface_size, 150);
    assert!((h.ctx.zoom_factor() - 1.5).abs() < 1e-3);
    // The page keeps its size on screen: points per page unit times pixels per point.
    let page_after = h.state().page_zoom() * h.ctx.zoom_factor();
    assert!((page_before - page_after).abs() < 1e-3, "{page_before} vs {page_after}");

    // Larger/Smaller step by 25 % and stay within 100–200 %.
    for (id, want) in [("ui.interfaceLarger", 175), ("ui.interfaceLarger", 200), ("ui.interfaceLarger", 200), ("ui.interfaceSmaller", 175)] {
        h.state_mut().execute(id, json!({})).unwrap();
        assert_eq!(h.state().prefs.appearance.interface_size, want, "{id}");
    }
    h.state_mut().execute("ui.interfaceSize", json!({"percent": 130})).unwrap();
    h.state_mut().execute("ui.interfaceSmaller", json!({})).unwrap();
    assert_eq!(h.state().prefs.appearance.interface_size, 125);
    h.state_mut().execute("ui.interfaceSize", json!({"percent": 130})).unwrap();
    h.state_mut().execute("ui.interfaceLarger", json!({})).unwrap();
    assert_eq!(h.state().prefs.appearance.interface_size, 150);

    // Hostile sizes are clamped or refused, never a panic.
    for (p, want) in [(json!(-5), 100), (json!(1e300), 200), (json!(99999), 200)] {
        h.state_mut().execute("ui.interfaceSize", json!({"percent": p})).unwrap();
        assert_eq!(h.state().prefs.appearance.interface_size, want);
    }
    assert!(h.state_mut().execute("ui.interfaceSize", json!({"percent": "big"})).is_err());
    // No size: back to 100 %.
    h.state_mut().execute("ui.interfaceSize", json!({})).unwrap();
    step(&mut h, 2);
    assert_eq!(h.state().prefs.appearance.interface_size, 100);
    assert!((h.ctx.zoom_factor() - 1.0).abs() < 1e-3);
}

#[test]
fn theme_can_follow_the_system() {
    use draftcraft_ui_egui::persist::Theme;
    let mut app = ScriptApp::new(Session::sample());
    app.dialogs = false;
    let mut h = harness(app);
    step(&mut h, 2);
    h.state_mut().execute("ui.theme.dark", json!({})).unwrap();
    assert_eq!(h.state().prefs.appearance.theme, Theme::Dark);
    assert!(h.state().dark);
    h.state_mut().execute("ui.theme.system", json!({})).unwrap();
    assert_eq!(h.state().prefs.appearance.theme, Theme::System);
    h.state_mut().execute("ui.theme.light", json!({})).unwrap();
    assert_eq!(h.state().prefs.appearance.theme, Theme::Light);
    assert!(!h.state().dark);
    let cmds = h.state().describe_commands();
    let size = cmds.as_array().unwrap().iter().find(|c| c["id"] == "ui.interfaceSize").unwrap();
    assert!(size["params"].as_str().unwrap().contains("percent"));
}
