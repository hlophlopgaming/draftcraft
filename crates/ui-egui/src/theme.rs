//! The design system: colour, type, spacing and shape tokens for the light and dark themes.
//!
//! Direction C · Focus (chosen by the owner in the issue "Redesign: choose a direction";
//! mockups in `docs/design/focus.html`, rules in `docs/design/README.md`): warm paper and desk
//! tones, one terracotta accent, chrome only where the writer looks for it. Every colour the
//! interface paints comes from [`Tokens`]; every size from [`type_scale`], [`space`] and
//! [`radius`]. The tests at the bottom check the contrast of each text/background pair.

use egui::{Color32, CornerRadius, Shadow, Stroke, Style, TextStyle, Visuals};

/// Type scale for the interface (points). The page itself uses Courier Badi at the script's
/// own size; these are for chrome, panels, cards and reports.
pub mod type_scale {
    /// Tiny labels squeezed into a strip (story map ticks).
    pub const MICRO: f32 = 9.5;
    /// Small caps labels, tags, captions.
    pub const CAPTION: f32 = 10.5;
    /// Secondary text: counts, page numbers, hints.
    pub const SMALL: f32 = 11.0;
    /// Body text and buttons.
    pub const BODY: f32 = 13.0;
    /// Titles inside a panel or card being edited.
    pub const SUBTITLE: f32 = 15.0;
    /// Panel and dialog headings.
    pub const HEADING: f32 = 17.0;
    /// Big numbers (statistics).
    pub const DISPLAY: f32 = 24.0;
}

/// Spacing steps (points): a 4-point grid with a 2-point half step.
pub mod space {
    pub const XXS: f32 = 2.0;
    pub const XS: f32 = 4.0;
    pub const S: f32 = 8.0;
    pub const M: f32 = 12.0;
    pub const L: f32 = 16.0;
    pub const XL: f32 = 24.0;
    pub const XXL: f32 = 32.0;
}

/// Corner radii (points).
pub mod radius {
    /// Buttons, fields, chips' inner parts.
    pub const S: u8 = 8;
    /// Menus, list rows, notes.
    pub const M: u8 = 10;
    /// Floating panels, popups, windows.
    pub const L: u8 = 14;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tokens {
    pub dark: bool,
    /// Title bar, menu bar, toolbar, status bar.
    pub chrome: Color32,
    /// Hairlines between chrome areas and around floating panels (the mockups' `--line`).
    pub chrome_line: Color32,
    /// Sidebars and docked panels.
    pub panel: Color32,
    /// Popups, floating panels, the command palette, the HUD pill.
    pub pop: Color32,
    /// Text fields, segmented controls, bars' empty track.
    pub field: Color32,
    /// The desk behind the pages.
    pub desk: Color32,
    pub paper: Color32,
    pub paper_edge: Color32,
    /// Script text on the page.
    pub ink: Color32,
    /// Page numbers, (MORE), scene numbers, the element label in the margin.
    pub ink_soft: Color32,
    /// Text outside the current speech in typewriter mode; placeholder lines.
    pub ink_dim: Color32,
    /// Interface text.
    pub text: Color32,
    pub text_soft: Color32,
    pub accent: Color32,
    /// A tint of the accent: the selected row, the hovered rail button.
    pub accent_weak: Color32,
    /// Text and icons drawn on an `accent` fill.
    pub on_accent: Color32,
    pub selection: Color32,
    pub caret: Color32,
    /// Removed text, misspellings, errors.
    pub danger: Color32,
    /// Added text, a goal reached.
    pub success: Color32,
    /// Script notes (sticky-note yellow, toned for the theme).
    pub note: Color32,
    /// Dims the desk behind a modal panel.
    pub scrim: Color32,
    /// Shadow under paper, cards and popups.
    pub shadow: Color32,
    pub card: Color32,
    /// Index card bands and act colours.
    pub card_band: [Color32; 4],
    /// Categorical chart colours (characters, series), in order.
    pub chart: [Color32; 5],
    /// Heat map ramp, from "nothing" to "a lot".
    pub heat: [Color32; 5],
}

const fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

const fn rgba(hex: u32, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied_const((hex >> 16) as u8, (hex >> 8) as u8, hex as u8, alpha)
}

/// The Focus palette, light.
pub const LIGHT: Tokens = Tokens {
    dark: false,
    chrome: rgb(0xf4f1ec),
    chrome_line: rgb(0xe4dfd6),
    panel: rgb(0xfbfaf7),
    pop: rgb(0xfffefb),
    field: rgb(0xebe6dd),
    desk: rgb(0xf4f1ec),
    paper: rgb(0xfffefb),
    paper_edge: rgb(0xe4dfd6),
    ink: rgb(0x1d1b18),
    // Darker than the mockups' #9a948a so page numbers keep 3:1 on the paper.
    ink_soft: rgb(0x8a8378),
    ink_dim: rgb(0xc9c2b6),
    text: rgb(0x2a2621),
    // Darker than the mockups' #857e73 so secondary text keeps 4.5:1 on every surface.
    text_soft: rgb(0x6a6359),
    accent: rgb(0xb4532a),
    accent_weak: rgb(0xf6e4da),
    on_accent: rgb(0xfffefb),
    selection: rgba(0xb4532a, 46),
    caret: rgb(0xb4532a),
    danger: rgb(0xb8322a),
    success: rgb(0x2f7a45),
    note: rgb(0xf8efd5),
    scrim: rgba(0x281e14, 26),
    shadow: rgba(0x3c2814, 20),
    card: rgb(0xfffefb),
    card_band: [rgb(0xd07a3a), rgb(0x3f8f7a), rgb(0x7a62c7), rgb(0xc0508f)],
    chart: [rgb(0xd07a3a), rgb(0x3f8f7a), rgb(0x7a62c7), rgb(0xc0508f), rgb(0x5b8bd0)],
    heat: [rgb(0xebe6dd), rgb(0xefc9b3), rgb(0xe09a74), rgb(0xc86a3e), rgb(0x9c4420)],
};

/// The Focus palette, dark.
pub const DARK: Tokens = Tokens {
    dark: true,
    chrome: rgb(0x181614),
    chrome_line: rgb(0x35302a),
    panel: rgb(0x1f1c19),
    pop: rgb(0x26221e),
    field: rgb(0x2b2722),
    desk: rgb(0x181614),
    paper: rgb(0x211e1b),
    paper_edge: rgb(0x35302a),
    ink: rgb(0xe6e0d6),
    ink_soft: rgb(0x8a8278),
    ink_dim: rgb(0x4a443d),
    text: rgb(0xece6dc),
    // Lighter than the mockups' #988f83 so secondary text keeps 4.5:1 on popups and fields.
    text_soft: rgb(0xa39a8e),
    accent: rgb(0xe58a5c),
    accent_weak: rgb(0x3d2a20),
    on_accent: rgb(0x181614),
    selection: rgba(0xe58a5c, 66),
    caret: rgb(0xf0a07a),
    danger: rgb(0xef7b72),
    success: rgb(0x6cc28a),
    note: rgb(0x3a3223),
    scrim: rgba(0x000000, 90),
    shadow: rgba(0x000000, 90),
    card: rgb(0x26221e),
    card_band: [rgb(0xe39257), rgb(0x5fb39c), rgb(0xa08ce6), rgb(0xdf74ae)],
    chart: [rgb(0xe39257), rgb(0x5fb39c), rgb(0xa08ce6), rgb(0xdf74ae), rgb(0x7ea9ea)],
    heat: [rgb(0x2b2722), rgb(0x4a3226), rgb(0x7a4a31), rgb(0xb0643d), rgb(0xe58a5c)],
};

impl Tokens {
    pub fn get(dark: bool) -> Tokens {
        if dark { DARK } else { LIGHT }
    }

    /// egui visuals in our colours and shapes.
    pub fn visuals(&self) -> Visuals {
        let mut v = if self.dark { Visuals::dark() } else { Visuals::light() };
        v.panel_fill = self.chrome;
        v.window_fill = self.pop;
        v.window_stroke = Stroke::new(1.0, self.chrome_line);
        v.faint_bg_color = self.panel;
        v.extreme_bg_color = self.field;
        v.code_bg_color = self.field;
        v.selection.bg_fill = self.accent;
        v.selection.stroke.color = self.on_accent;
        v.hyperlink_color = self.accent;
        v.text_cursor.stroke.color = self.caret;
        // No `override_text_color`: it would also win over `selection.stroke` and put body text
        // on the accent fill of a selected tab. Text gets its colour from the widget states below.
        v.override_text_color = None;
        v.window_corner_radius = CornerRadius::same(radius::L);
        v.menu_corner_radius = CornerRadius::same(radius::M);
        v.window_shadow = Shadow { offset: [0, 10], blur: 30, spread: 0, color: self.shadow };
        v.popup_shadow = Shadow { offset: [0, 6], blur: 20, spread: 0, color: self.shadow };
        let w = &mut v.widgets;
        w.noninteractive.bg_fill = self.panel;
        w.noninteractive.weak_bg_fill = self.panel;
        w.noninteractive.bg_stroke.color = self.chrome_line;
        w.noninteractive.fg_stroke.color = self.text;
        w.inactive.bg_fill = self.field;
        w.inactive.weak_bg_fill = self.field;
        w.inactive.fg_stroke.color = self.text;
        w.hovered.bg_fill = self.accent_weak;
        w.hovered.weak_bg_fill = self.accent_weak;
        w.hovered.bg_stroke.color = self.chrome_line;
        w.hovered.fg_stroke.color = self.text;
        w.active.bg_fill = self.accent_weak;
        w.active.weak_bg_fill = self.accent_weak;
        w.active.bg_stroke.color = self.accent;
        w.active.fg_stroke.color = self.text;
        w.open.bg_fill = self.accent_weak;
        w.open.weak_bg_fill = self.accent_weak;
        w.open.bg_stroke.color = self.chrome_line;
        w.open.fg_stroke.color = self.text;
        for state in [&mut w.noninteractive, &mut w.inactive, &mut w.hovered, &mut w.active, &mut w.open] {
            state.corner_radius = CornerRadius::same(radius::S);
        }
        v
    }

    /// Apply the visuals, the type scale and the spacing to a style.
    pub fn apply(&self, style: &mut Style) {
        style.visuals = self.visuals();
        apply_metrics(style);
    }
}

/// The type scale and spacing, the same in both themes.
pub fn apply_metrics(style: &mut Style) {
    use egui::{FontFamily, FontId};
    for (text_style, size, family) in [
        (TextStyle::Small, type_scale::SMALL, FontFamily::Proportional),
        (TextStyle::Body, type_scale::BODY, FontFamily::Proportional),
        (TextStyle::Button, type_scale::BODY, FontFamily::Proportional),
        (TextStyle::Monospace, type_scale::BODY, FontFamily::Monospace),
        (TextStyle::Heading, type_scale::HEADING, FontFamily::Proportional),
    ] {
        style.text_styles.insert(text_style, FontId::new(size, family));
    }
    let s = &mut style.spacing;
    s.item_spacing = egui::vec2(space::S, space::XS);
    s.button_padding = egui::vec2(space::S, space::XXS + 1.0);
    s.window_margin = egui::Margin::same(space::M as i8);
    s.menu_margin = egui::Margin::same(space::XS as i8);
}

/// WCAG 2 contrast ratio of two opaque colours (1 to 21).
pub fn contrast(a: Color32, b: Color32) -> f32 {
    fn luminance(c: Color32) -> f32 {
        let lin = |v: u8| {
            let s = v as f32 / 255.0;
            if s <= 0.040_45 { s / 12.92 } else { ((s + 0.055) / 1.055).powf(2.4) }
        };
        0.2126 * lin(c.r()) + 0.7152 * lin(c.g()) + 0.0722 * lin(c.b())
    }
    let (la, lb) = (luminance(a), luminance(b));
    let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// WCAG AA: 4.5:1 for text, 3:1 for large text and for the parts of controls.
    const TEXT: f32 = 4.5;
    const UI: f32 = 3.0;

    fn check(t: &Tokens, name: &str, fg: Color32, bg: Color32, min: f32, failures: &mut Vec<String>) {
        let r = contrast(fg, bg);
        if r < min {
            failures.push(format!("{} theme: {name} is {r:.2}:1, needs {min}:1", if t.dark { "dark" } else { "light" }));
        }
    }

    #[test]
    fn contrast_meets_wcag_aa() {
        let mut failures = Vec::new();
        for t in [LIGHT, DARK] {
            let surfaces = [
                ("chrome", t.chrome),
                ("panel", t.panel),
                ("pop", t.pop),
                ("field", t.field),
                ("desk", t.desk),
                ("card", t.card),
                ("accent_weak", t.accent_weak),
                ("note", t.note),
            ];
            for (s, bg) in surfaces {
                check(&t, &format!("text on {s}"), t.text, bg, TEXT, &mut failures);
                check(&t, &format!("text_soft on {s}"), t.text_soft, bg, TEXT, &mut failures);
            }
            for (s, bg) in [("chrome", t.chrome), ("panel", t.panel), ("pop", t.pop), ("paper", t.paper)] {
                check(&t, &format!("accent on {s}"), t.accent, bg, UI, &mut failures);
            }
            check(&t, "ink on paper", t.ink, t.paper, 7.0, &mut failures);
            check(&t, "ink_soft on paper", t.ink_soft, t.paper, UI, &mut failures);
            check(&t, "caret on paper", t.caret, t.paper, UI, &mut failures);
            for (s, bg) in [("pop", t.pop), ("paper", t.paper)] {
                check(&t, &format!("danger on {s}"), t.danger, bg, TEXT, &mut failures);
                check(&t, &format!("success on {s}"), t.success, bg, TEXT, &mut failures);
            }
            check(&t, "on_accent on accent", t.on_accent, t.accent, TEXT, &mut failures);
            for (k, c) in t.card_band.iter().chain(t.chart.iter()).enumerate() {
                check(&t, &format!("chart colour {k} on pop"), *c, t.pop, UI, &mut failures);
            }
        }
        assert!(failures.is_empty(), "contrast below WCAG AA:\n{}", failures.join("\n"));
    }

    #[test]
    fn contrast_is_symmetric_and_bounded() {
        assert!((contrast(Color32::BLACK, Color32::WHITE) - 21.0).abs() < 0.01);
        assert!((contrast(Color32::WHITE, Color32::BLACK) - 21.0).abs() < 0.01);
        assert!((contrast(LIGHT.text, LIGHT.text) - 1.0).abs() < 0.001);
    }

    #[test]
    fn themes_differ() {
        assert_ne!(LIGHT, DARK);
        assert!(!LIGHT.dark && DARK.dark);
        assert_eq!(Tokens::get(true), DARK);
        assert_eq!(Tokens::get(false), LIGHT);
    }

    #[test]
    fn metrics_follow_the_scale() {
        let mut style = Style::default();
        DARK.apply(&mut style);
        assert_eq!(style.text_styles.get(&TextStyle::Body).map(|f| f.size), Some(type_scale::BODY));
        assert_eq!(style.text_styles.get(&TextStyle::Heading).map(|f| f.size), Some(type_scale::HEADING));
        assert_eq!(style.spacing.item_spacing, egui::vec2(space::S, space::XS));
        assert_eq!(style.visuals.window_fill, DARK.pop);
        assert_eq!(style.visuals.override_text_color, None, "selected tabs must use on_accent");
        assert_eq!(style.visuals.selection.stroke.color, DARK.on_accent);
        assert_eq!(style.visuals.widgets.noninteractive.fg_stroke.color, DARK.text);
        assert_eq!(style.visuals.widgets.inactive.corner_radius, CornerRadius::same(radius::S));
    }
}
