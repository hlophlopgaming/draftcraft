//! Dialogs in the Focus look (v0.6 "Redesigned dialogs"; direction C, `docs/design/README.md`).
//!
//! One frame for every dialog: a popup-coloured card with the large corner radius and a soft
//! shadow, a heading in the type scale with a close cross, roomy margins, and a footer whose
//! buttons sit on the right with the main action last, filled with the accent. Questions that
//! must be answered first (save before closing, recover after a crash) are modal: the desk
//! behind them dims and Escape or a click outside cancels. Everything is painted with
//! [`crate::theme`] tokens.

use crate::icons::{self, Icon};
use crate::theme::{Tokens, radius, space, type_scale};
use egui::{Align, Button, Context, CornerRadius, Frame, Id, Layout, Margin, Modal, Rect, Response, RichText, Sense, Shadow, Stroke, Ui, vec2};

/// The smallest width of a footer button, so a row of short labels lines up.
const BUTTON_MIN: egui::Vec2 = vec2(84.0, 28.0);

/// The card every dialog is drawn on.
pub fn frame(t: &Tokens) -> Frame {
    Frame::NONE
        .fill(t.pop)
        .stroke(Stroke::new(1.0, t.chrome_line))
        .corner_radius(CornerRadius::same(radius::L))
        .inner_margin(Margin::symmetric(space::XL as i8, space::L as i8))
        .shadow(Shadow { offset: [0, 12], blur: 36, spread: 0, color: t.shadow })
}

/// The heading row: the title, an optional line under it, and (when `closable`) a close cross on
/// the right. Returns true when the cross was clicked.
pub fn header(ui: &mut Ui, t: &Tokens, title: &str, subtitle: Option<&str>, closable: bool, close_label: &str) -> bool {
    let mut close = false;
    ui.horizontal(|ui| {
        ui.label(RichText::new(title).size(type_scale::HEADING).strong().color(t.text));
        if closable {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                close = close_button(ui, t, close_label).clicked();
            });
        }
    });
    if let Some(s) = subtitle {
        ui.add_space(space::XXS);
        ui.label(RichText::new(s).size(type_scale::SMALL).color(t.text_soft));
    }
    ui.add_space(space::M);
    close
}

/// The close cross of a dialog: a quiet icon that tints on hover.
pub fn close_button(ui: &mut Ui, t: &Tokens, label: &str) -> Response {
    let (r, resp) = ui.allocate_exact_size(vec2(24.0, 24.0), Sense::click());
    let label = label.to_string();
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, label.clone()));
    if resp.hovered() {
        ui.painter().rect_filled(r, CornerRadius::same(radius::S), t.accent_weak);
    }
    let c = if resp.hovered() { t.text } else { t.text_soft };
    icons::paint(ui, Icon::Close, Rect::from_center_size(r.center(), vec2(14.0, 14.0)), c);
    resp.on_hover_text(label)
}

/// The main action of a dialog: accent fill, text in `on_accent`.
pub fn primary(ui: &mut Ui, t: &Tokens, text: &str) -> Response {
    ui.add(
        Button::new(RichText::new(text).color(t.on_accent).strong())
            .fill(t.accent)
            .stroke(Stroke::NONE)
            .corner_radius(CornerRadius::same(radius::S))
            .min_size(BUTTON_MIN),
    )
}

/// Any other dialog button: the field colour, the accent tint on hover.
pub fn secondary(ui: &mut Ui, t: &Tokens, text: &str) -> Response {
    ui.add(Button::new(RichText::new(text).color(t.text)).stroke(Stroke::NONE).corner_radius(CornerRadius::same(radius::S)).min_size(BUTTON_MIN))
}

/// The footer: a hairline, then `left` (extra actions) on the left and `right` on the right.
/// `right` is laid out right to left, so add the main action first: it ends up last.
pub fn footer(ui: &mut Ui, t: &Tokens, left: impl FnOnce(&mut Ui), right: impl FnOnce(&mut Ui)) {
    ui.add_space(space::L);
    let (line, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter().hline(line.x_range(), line.center().y, Stroke::new(1.0, t.chrome_line));
    ui.add_space(space::M);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = space::S;
        left(ui);
        ui.with_layout(Layout::right_to_left(Align::Center), right);
    });
}

/// A floating dialog the writer can keep open while writing (Title Page): the Focus card with a
/// heading and a close cross, no system title bar. `open` turns false when it is closed.
pub fn window<R>(
    ctx: &Context,
    t: &Tokens,
    id: &str,
    title: &str,
    close_label: &str,
    open: &mut bool,
    width: f32,
    body: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    let mut closed = false;
    let out = egui::Window::new(title)
        .id(Id::unique(("dialog", id)))
        .title_bar(false)
        .frame(frame(t))
        .resizable(false)
        .collapsible(false)
        .default_width(width)
        .pivot(egui::Align2::CENTER_TOP)
        .default_pos(ctx.content_rect().center_top() + vec2(0.0, 90.0))
        .show(ctx, |ui| {
            ui.set_width(width);
            closed = header(ui, t, title, None, true, close_label);
            body(ui)
        })
        .and_then(|r| r.inner);
    if closed {
        *open = false;
    }
    out
}

/// A question that has to be answered first: the desk dims, the card sits in the middle.
/// Returns the body's value and whether the writer dismissed it (Escape or a click outside).
pub fn modal<R>(ctx: &Context, t: &Tokens, id: &str, title: &str, message: &str, body: impl FnOnce(&mut Ui) -> R) -> (R, bool) {
    let r = Modal::new(Id::unique(("modal", id))).frame(frame(t)).backdrop_color(t.scrim).show(ctx, |ui| {
        ui.set_max_width(440.0);
        header(ui, t, title, None, false, "");
        ui.label(RichText::new(message).size(type_scale::BODY).color(t.text));
        body(ui)
    });
    let dismissed = r.should_close();
    (r.inner, dismissed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::{DARK, LIGHT, contrast};

    #[test]
    fn the_main_button_reads_in_both_themes() {
        for t in [LIGHT, DARK] {
            assert!(contrast(t.on_accent, t.accent) >= 4.5, "{}: button text on the accent", t.dark);
            assert!(contrast(t.text, t.field) >= 4.5, "{}: secondary button text", t.dark);
            assert!(contrast(t.text, t.pop) >= 4.5, "{}: dialog text", t.dark);
        }
    }

    #[test]
    fn dialogs_lay_out_without_panicking() {
        let ctx = Context::default();
        let mut open = true;
        for t in [LIGHT, DARK] {
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
                let ctx = ui.ctx().clone();
                window(&ctx, &t, "test", "Title", "Close", &mut open, 400.0, |ui| {
                    footer(
                        ui,
                        &t,
                        |ui| {
                            ui.label("");
                        },
                        |ui| {
                            primary(ui, &t, "Done");
                            secondary(ui, &t, "");
                        },
                    );
                });
                let ((), dismissed) = modal(&ctx, &t, "q", "Question?", "", |ui| footer(ui, &t, |_| {}, |_| {}));
                assert!(!dismissed);
            });
            out.textures_delta.clear();
        }
        assert!(open);
    }
}
