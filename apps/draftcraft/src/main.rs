//! The DraftCraft desktop app.
//!
//! Usage: `draftcraft [--new | --sample] [--config-dir <dir>] [--control <port>] [file.fountain|file.fdx|file.docx|file.celtx|file.pdf]`
//!
//! With no file it opens the sample script (`--new` starts blank). Settings and data follow the
//! Craft suite's folder rules (craft-settings): `--config-dir`, then `DRAFTCRAFT_CONFIG_DIR`
//! (empty = isolated, nothing read or written), then a `portable` marker, then the platform
//! folder. `--control <port>` (or `DRAFTCRAFT_CONTROL_PORT`) starts a localhost JSON-lines
//! control server; every request carries the key the app writes to
//! `<data>/control-key.local-<port>` (see docs/control-protocol.md).
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

mod control_server;
mod printing;
mod tts;

use draftcraft_engine::Session;
use draftcraft_ui_egui::ScriptApp;
use serde_json::json;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Set by the signal handler: the process is being terminated.
static TERMINATING: AtomicBool = AtomicBool::new(false);

struct App(ScriptApp, Option<control_server::KeyFile>);

impl eframe::App for App {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if TERMINATING.load(Ordering::SeqCst) {
            // SIGTERM/SIGINT/SIGHUP: keep unsaved work as a recovery copy, drop the key, exit.
            self.0.emergency_save();
            if let Some(k) = &self.1 {
                k.remove();
            }
            std::process::exit(0);
        }
        self.0.logic(ctx);
        if std::mem::take(&mut self.0.close_wanted) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        let closing = ctx.input(|i| i.viewport().close_requested());
        if closing && !self.0.close_requested() {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
        } else if self.0.quit_requested {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
    fn raw_input_hook(&mut self, _ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.0.raw_input_hook(raw);
    }
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.0.ui(ui);
    }
    fn on_exit(&mut self) {
        self.0.on_exit();
        if let Some(k) = &self.1 {
            k.remove();
        }
    }
}

/// The window icon: the stag-beetle app icon (`docs/images/icon.svg`), embedded as a 256 px PNG.
/// If it cannot be decoded the window keeps the platform's default icon.
fn icon() -> Option<egui::IconData> {
    let img = image::load_from_memory_with_format(include_bytes!("../assets/icon-256.png"), image::ImageFormat::Png).ok()?.into_rgba8();
    let (width, height) = img.dimensions();
    Some(egui::IconData { rgba: img.into_raw(), width, height })
}

/// On SIGTERM/SIGINT/SIGHUP, wake the UI so it saves and exits; if it does not within 3 s
/// (frozen), remove the key file and exit anyway.
fn install_signal_handler(ctx: Arc<Mutex<Option<egui::Context>>>, key: Arc<Mutex<Option<control_server::KeyFile>>>) {
    let r = ctrlc::set_handler(move || {
        TERMINATING.store(true, Ordering::SeqCst);
        if let Some(c) = ctx.lock().ok().and_then(|g| g.clone()) {
            c.request_repaint();
        }
        let key = key.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(3));
            if let Some(k) = key.lock().ok().and_then(|g| g.clone()) {
                k.remove();
            }
            std::process::exit(1);
        });
    });
    if let Err(e) = r {
        eprintln!("draftcraft: no signal handler ({e})");
    }
}

fn main() -> eframe::Result<()> {
    let ctx_slot: Arc<Mutex<Option<egui::Context>>> = Arc::new(Mutex::new(None));
    let key_slot: Arc<Mutex<Option<control_server::KeyFile>>> = Arc::new(Mutex::new(None));
    install_signal_handler(ctx_slot.clone(), key_slot.clone());
    let mut control_port: Option<u16> = std::env::var("DRAFTCRAFT_CONTROL_PORT").ok().and_then(|p| p.parse().ok());
    let mut file: Option<String> = None;
    let mut blank = false;
    let mut config_dir: Option<std::path::PathBuf> = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--control" => control_port = args.next().and_then(|p| p.parse().ok()),
            "--config-dir" => config_dir = args.next().map(std::path::PathBuf::from),
            "--new" => blank = true,
            "--sample" => blank = false,
            "--help" | "-h" => {
                println!(
                    "draftcraft [--new | --sample] [--config-dir <dir>] [--control <port>] [file.fountain|file.fdx|file.docx|file.celtx|file.pdf]"
                );
                return Ok(());
            }
            other => file = Some(other.to_string()),
        }
    }
    let mut session = if blank { Session::new(Default::default()) } else { Session::sample() };
    if let Some(f) = &file
        && let Err(e) = session.run("file.open", &json!({"path": f}))
    {
        eprintln!("draftcraft: {e}");
    }
    let mut options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1440.0, 900.0]).with_min_inner_size([720.0, 480.0]).with_title("DraftCraft"),
        ..Default::default()
    };
    if let Some(icon) = icon() {
        options.viewport = options.viewport.with_icon(icon);
    }
    // macOS: winit's default app menu has a Quit item (Cmd+Q) that calls `terminate:` directly,
    // which would skip the unsaved-changes prompt. Without it, Cmd+Q reaches the app as a
    // shortcut (ui.quit) and goes through the same prompt as the close button.
    #[cfg(target_os = "macos")]
    {
        options.event_loop_builder = Some(Box::new(|b| {
            use winit::platform::macos::EventLoopBuilderExtMacOS;
            b.with_default_menu(false);
        }));
    }
    eframe::run_native(
        "DraftCraft",
        options,
        Box::new(move |cc| {
            if let Ok(mut g) = ctx_slot.lock() {
                *g = Some(cc.egui_ctx.clone());
            }
            let mut app = ScriptApp::new(session);
            app.dark = cc.egui_ctx.system_theme() == Some(egui::Theme::Dark);
            let folders = draftcraft_control_key::folders(config_dir);
            let data = folders.data.clone();
            app = app.with_folders(folders);
            // Read aloud with the system's voices (and whisper.cpp dictation when installed).
            app.speaker = tts::detect().map(|s| Box::new(s) as Box<dyn draftcraft_ui_egui::speech::SpeechHost>);
            // Print to the system's printers (CUPS, or the Windows print action).
            app.printer = printing::detect().map(|p| Box::new(p) as Box<dyn draftcraft_ui_egui::printing::PrintHost>);
            let mut key_file = None;
            if let Some(port) = control_port
                && let Some((rx, kf)) = control_server::start(port, cc.egui_ctx.clone(), data.as_deref())
            {
                app = app.with_control(rx);
                if let Ok(mut g) = key_slot.lock() {
                    g.clone_from(&kf);
                }
                key_file = kf;
            }
            Ok(Box::new(App(app, key_file)))
        }),
    )
}
