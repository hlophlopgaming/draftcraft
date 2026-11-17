//! Interface icons, drawn in code (original; see ATTRIBUTION.md).
//!
//! The Focus line style (`docs/design/focus.html`, chosen in issue #4): every icon is a set of
//! thin strokes on a 24-unit grid, 1.6 units wide, with round ends and rounded corners, never
//! filled. An icon is data ([`shapes`]), so tests can check it without drawing, and the same
//! table serves every size: the toolbar today, the tool rail and panels of the new layout later.

use egui::{Color32, CornerRadius, Pos2, Rect, Sense, Stroke, StrokeKind, Ui, vec2};

use crate::theme;

/// The grid icons are drawn on, in units (like an SVG `viewBox="0 0 24 24"`).
pub const GRID: f32 = 24.0;
/// Stroke width in grid units (1.2 px at the toolbar's 18 px).
pub const STROKE: f32 = 1.6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Icon {
    New,
    Open,
    Save,
    Pdf,
    Page,
    Cards,
    Reports,
    Outline,
    StoryMap,
    Navigator,
    Find,
    Moon,
    Sun,
    Dual,
    Numbers,
    Title,
    Undo,
    Redo,
    Beats,
    Split,
}

impl Icon {
    /// Every icon, for tests and the icon sheet.
    pub const ALL: [Icon; 20] = [
        Icon::New,
        Icon::Open,
        Icon::Save,
        Icon::Pdf,
        Icon::Page,
        Icon::Cards,
        Icon::Reports,
        Icon::Outline,
        Icon::StoryMap,
        Icon::Navigator,
        Icon::Find,
        Icon::Moon,
        Icon::Sun,
        Icon::Dual,
        Icon::Numbers,
        Icon::Title,
        Icon::Undo,
        Icon::Redo,
        Icon::Beats,
        Icon::Split,
    ];
}

/// One piece of a path: a point, or an arc around `(cx, cy)` of radius `r` from angle `a0` to
/// `a1` (degrees, clockwise on screen because y points down; `a1 < a0` runs the other way).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Seg {
    Pt(f32, f32),
    Arc(f32, f32, f32, f32, f32),
}

/// One stroke of an icon, in grid units.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Prim {
    /// Straight segments through the points; corners rounded by `round`; `closed` joins the
    /// last point to the first.
    Poly(&'static [(f32, f32)], bool, f32),
    /// Points and arcs joined in order; `closed` joins the end to the start.
    Path(&'static [Seg], bool),
    /// A rectangle `x, y, w, h` with corner radius `r`.
    Rect(f32, f32, f32, f32, f32),
    /// A circle at `cx, cy` of radius `r`.
    Circle(f32, f32, f32),
    /// A dot (a round pen tip set down once), e.g. the bullets of the outline.
    Dot(f32, f32),
}

use Prim::{Circle, Dot, Path, Poly, Rect as R};
use Seg::{Arc, Pt};

const PAGE: Prim = R(5.0, 3.0, 14.0, 18.0, 2.0);

/// The strokes of `icon`. [`Icon::Redo`] is [`Icon::Undo`] mirrored ([`mirrored`]).
pub fn shapes(icon: Icon) -> &'static [Prim] {
    match icon {
        Icon::New => &[PAGE, Poly(&[(12.0, 9.0), (12.0, 15.0)], false, 0.0), Poly(&[(9.0, 12.0), (15.0, 12.0)], false, 0.0)],
        Icon::Open => &[
            Poly(&[(3.0, 5.0), (9.0, 5.0), (11.0, 7.0), (21.0, 7.0), (21.0, 19.0), (3.0, 19.0)], true, 2.0),
            Poly(&[(3.0, 10.0), (21.0, 10.0)], false, 0.0),
        ],
        Icon::Save => &[
            Poly(&[(4.0, 4.0), (17.0, 4.0), (20.0, 7.0), (20.0, 20.0), (4.0, 20.0)], true, 2.0),
            Poly(&[(8.0, 4.0), (8.0, 8.5), (15.0, 8.5), (15.0, 4.0)], false, 1.0),
            Poly(&[(8.0, 20.0), (8.0, 14.0), (16.0, 14.0), (16.0, 20.0)], false, 1.0),
        ],
        Icon::Pdf => &[
            Poly(&[(5.0, 3.0), (14.0, 3.0), (19.0, 8.0), (19.0, 21.0), (5.0, 21.0)], true, 2.0),
            Poly(&[(14.0, 3.0), (14.0, 8.0), (19.0, 8.0)], false, 0.0),
            Poly(&[(12.0, 10.0), (12.0, 17.0)], false, 0.0),
            Poly(&[(9.0, 14.0), (12.0, 17.0), (15.0, 14.0)], false, 0.0),
        ],
        Icon::Page => &[
            PAGE,
            Poly(&[(9.0, 8.0), (15.0, 8.0)], false, 0.0),
            Poly(&[(9.0, 12.0), (15.0, 12.0)], false, 0.0),
            Poly(&[(9.0, 16.0), (13.0, 16.0)], false, 0.0),
        ],
        Icon::Title => &[PAGE, Poly(&[(9.0, 10.0), (15.0, 10.0)], false, 0.0), Poly(&[(10.5, 14.0), (13.5, 14.0)], false, 0.0)],
        Icon::Cards => &[R(3.0, 4.0, 8.0, 7.0, 1.5), R(13.0, 4.0, 8.0, 7.0, 1.5), R(3.0, 13.0, 8.0, 7.0, 1.5), R(13.0, 13.0, 8.0, 7.0, 1.5)],
        Icon::Reports => &[
            Poly(&[(5.0, 20.0), (5.0, 10.0)], false, 0.0),
            Poly(&[(10.0, 20.0), (10.0, 4.0)], false, 0.0),
            Poly(&[(15.0, 20.0), (15.0, 13.0)], false, 0.0),
            Poly(&[(20.0, 20.0), (20.0, 16.0)], false, 0.0),
        ],
        Icon::Outline => &[
            Dot(5.0, 6.0),
            Dot(5.0, 12.0),
            Dot(5.0, 18.0),
            Poly(&[(9.0, 6.0), (20.0, 6.0)], false, 0.0),
            Poly(&[(11.0, 12.0), (20.0, 12.0)], false, 0.0),
            Poly(&[(11.0, 18.0), (20.0, 18.0)], false, 0.0),
        ],
        Icon::StoryMap => &[
            // Bands (acts) over a row of scene blocks.
            R(3.0, 4.0, 18.0, 5.0, 1.5),
            Poly(&[(12.0, 4.0), (12.0, 9.0)], false, 0.0),
            R(3.0, 12.0, 5.0, 8.0, 1.5),
            R(10.0, 12.0, 4.0, 8.0, 1.5),
            R(16.0, 12.0, 5.0, 8.0, 1.5),
        ],
        Icon::Navigator => &[
            Poly(&[(4.0, 6.0), (20.0, 6.0)], false, 0.0),
            Poly(&[(4.0, 12.0), (20.0, 12.0)], false, 0.0),
            Poly(&[(4.0, 18.0), (14.0, 18.0)], false, 0.0),
        ],
        Icon::Find => &[Circle(11.0, 11.0, 6.0), Poly(&[(20.0, 20.0), (15.5, 15.5)], false, 0.0)],
        // A crescent: the outer edge of one circle, then the bite of another, meeting at
        // (20, 14.5) and (9.5, 4).
        Icon::Moon => &[Path(&[Arc(12.64, 11.36, 8.0, 23.1, 246.9), Arc(16.86, 7.14, 8.0, 203.1, 66.9)], true)],
        Icon::Sun => &[
            Circle(12.0, 12.0, 4.0),
            Poly(&[(12.0, 2.0), (12.0, 4.0)], false, 0.0),
            Poly(&[(12.0, 20.0), (12.0, 22.0)], false, 0.0),
            Poly(&[(2.0, 12.0), (4.0, 12.0)], false, 0.0),
            Poly(&[(20.0, 12.0), (22.0, 12.0)], false, 0.0),
            Poly(&[(4.9, 4.9), (6.3, 6.3)], false, 0.0),
            Poly(&[(17.7, 17.7), (19.1, 19.1)], false, 0.0),
            Poly(&[(4.9, 19.1), (6.3, 17.7)], false, 0.0),
            Poly(&[(17.7, 6.3), (19.1, 4.9)], false, 0.0),
        ],
        Icon::Dual => &[
            // Two speeches side by side: a short cue over each column of lines.
            Poly(&[(5.5, 5.0), (8.5, 5.0)], false, 0.0),
            Poly(&[(3.0, 10.0), (10.0, 10.0)], false, 0.0),
            Poly(&[(3.0, 14.0), (10.0, 14.0)], false, 0.0),
            Poly(&[(3.0, 18.0), (8.0, 18.0)], false, 0.0),
            Poly(&[(15.5, 5.0), (18.5, 5.0)], false, 0.0),
            Poly(&[(14.0, 10.0), (21.0, 10.0)], false, 0.0),
            Poly(&[(14.0, 14.0), (21.0, 14.0)], false, 0.0),
            Poly(&[(14.0, 18.0), (19.0, 18.0)], false, 0.0),
        ],
        Icon::Numbers => &[
            Poly(&[(10.0, 4.0), (8.0, 20.0)], false, 0.0),
            Poly(&[(16.0, 4.0), (14.0, 20.0)], false, 0.0),
            Poly(&[(4.5, 9.0), (19.5, 9.0)], false, 0.0),
            Poly(&[(4.5, 15.0), (19.5, 15.0)], false, 0.0),
        ],
        Icon::Undo | Icon::Redo => &[
            Poly(&[(9.0, 14.0), (4.0, 9.0), (9.0, 4.0)], false, 0.5),
            Path(&[Pt(4.0, 9.0), Pt(14.5, 9.0), Arc(14.5, 14.5, 5.5, -90.0, 90.0), Pt(11.0, 20.0)], false),
        ],
        Icon::Beats => &[
            Circle(6.0, 12.0, 2.0),
            Circle(12.0, 7.0, 2.0),
            Circle(18.0, 14.0, 2.0),
            Poly(&[(7.5, 10.5), (10.5, 8.5)], false, 0.0),
            Poly(&[(13.6, 8.6), (16.6, 12.6)], false, 0.0),
        ],
        Icon::Split => &[
            R(3.0, 4.0, 18.0, 16.0, 2.0),
            Poly(&[(12.0, 4.0), (12.0, 20.0)], false, 0.0),
            Poly(&[(6.0, 9.0), (9.0, 9.0)], false, 0.0),
            Poly(&[(6.0, 13.0), (9.0, 13.0)], false, 0.0),
            Poly(&[(15.0, 9.0), (18.0, 9.0)], false, 0.0),
        ],
    }
}

/// Whether `icon` is drawn as the mirror image of its table (left and right swapped).
pub fn mirrored(icon: Icon) -> bool {
    icon == Icon::Redo
}

/// Points along an arc, at most 10° apart (and never more than 64 of them).
fn arc_points(cx: f32, cy: f32, r: f32, a0: f32, a1: f32, out: &mut Vec<(f32, f32)>) {
    let span = a1 - a0;
    let n = (span.abs() / 10.0).ceil().clamp(1.0, 64.0) as usize;
    for k in 0..=n {
        let a = (a0 + span * k as f32 / n as f32).to_radians();
        out.push((cx + r * a.cos(), cy + r * a.sin()));
    }
}

/// `pts` with every corner (each end too when `closed`) replaced by a short curve of radius
/// about `round`, as SVG's rounded joins look at icon sizes.
fn round_corners(pts: &[(f32, f32)], closed: bool, round: f32) -> Vec<(f32, f32)> {
    let n = pts.len();
    if round <= 0.0 || n < 3 {
        return pts.to_vec();
    }
    let mut out = Vec::with_capacity(n * 5);
    for (i, &p) in pts.iter().enumerate() {
        let ends = !closed && (i == 0 || i + 1 == n);
        let (Some(&a), Some(&b)) = (pts.get((i + n - 1) % n), pts.get((i + 1) % n)) else {
            continue;
        };
        if ends {
            out.push(p);
            continue;
        }
        // Step back towards the previous point and on towards the next, by at most half of
        // either side, and curve between the two through the corner (a quadratic Bézier).
        let toward = |q: (f32, f32)| {
            let (dx, dy) = (q.0 - p.0, q.1 - p.1);
            let len = (dx * dx + dy * dy).sqrt();
            let t = if len > 0.0 { (round.min(len / 2.0)) / len } else { 0.0 };
            (p.0 + dx * t, p.1 + dy * t)
        };
        let (s, e) = (toward(a), toward(b));
        for k in 0..=4 {
            let t = k as f32 / 4.0;
            let u = 1.0 - t;
            out.push((u * u * s.0 + 2.0 * u * t * p.0 + t * t * e.0, u * u * s.1 + 2.0 * u * t * p.1 + t * t * e.1));
        }
    }
    out
}

/// The points of a path, in grid units.
fn path_points(segs: &[Seg]) -> Vec<(f32, f32)> {
    let mut out = Vec::new();
    for s in segs {
        match *s {
            Seg::Pt(x, y) => out.push((x, y)),
            Seg::Arc(cx, cy, r, a0, a1) => arc_points(cx, cy, r, a0, a1, &mut out),
        }
    }
    out
}

/// The extent of a primitive in grid units, stroke included: `(min x, min y, max x, max y)`.
pub fn bounds(p: &Prim) -> (f32, f32, f32, f32) {
    let h = STROKE / 2.0;
    let of = |pts: &[(f32, f32)]| {
        pts.iter()
            .fold((f32::MAX, f32::MAX, f32::MIN, f32::MIN), |(x0, y0, x1, y1), &(x, y)| (x0.min(x - h), y0.min(y - h), x1.max(x + h), y1.max(y + h)))
    };
    match *p {
        Prim::Poly(pts, _, _) => of(pts),
        Prim::Path(segs, _) => of(&path_points(segs)),
        Prim::Rect(x, y, w, ht, _) => (x - h, y - h, x + w + h, y + ht + h),
        Prim::Circle(cx, cy, r) => (cx - r - h, cy - r - h, cx + r + h, cy + r + h),
        Prim::Dot(x, y) => (x - h, y - h, x + h, y + h),
    }
}

/// Draw `icon` centred in `r` (in the largest square that fits) with colour `c`.
pub fn paint(ui: &Ui, icon: Icon, r: Rect, c: Color32) {
    let p = ui.painter();
    let side = r.width().min(r.height()).max(0.0);
    let k = side / GRID;
    let origin = r.center() - vec2(side, side) / 2.0;
    let flip = mirrored(icon);
    let at = |(x, y): (f32, f32)| -> Pos2 {
        let x = if flip { GRID - x } else { x };
        origin + vec2(x * k, y * k)
    };
    let w = (STROKE * k).max(1.0);
    let s = Stroke::new(w, c);
    // egui ends lines square; a dot of half the stroke width at each end rounds them, as the
    // mockups' `stroke-linecap: round` does.
    let cap = |q: Pos2| {
        p.circle_filled(q, w / 2.0, c);
    };
    let line = |pts: Vec<Pos2>, closed: bool| {
        if pts.len() < 2 {
            return;
        }
        if !closed {
            if let (Some(&a), Some(&b)) = (pts.first(), pts.last()) {
                cap(a);
                cap(b);
            }
            p.add(egui::Shape::line(pts, s));
        } else {
            p.add(egui::Shape::closed_line(pts, s));
        }
    };
    for prim in shapes(icon) {
        match *prim {
            Prim::Poly(pts, closed, round) => line(round_corners(pts, closed, round).into_iter().map(at).collect(), closed),
            Prim::Path(segs, closed) => line(path_points(segs).into_iter().map(at).collect(), closed),
            Prim::Rect(x, y, rw, rh, rr) => {
                let rect = Rect::from_two_pos(at((x, y)), at((x + rw, y + rh)));
                let cr = CornerRadius::from((rr * k).clamp(0.0, 255.0));
                p.rect_stroke(rect, cr, s, StrokeKind::Middle);
            }
            Prim::Circle(cx, cy, cr) => {
                p.circle_stroke(at((cx, cy)), cr * k, s);
            }
            Prim::Dot(x, y) => {
                p.circle_filled(at((x, y)), w * 0.75, c);
            }
        }
    }
}

/// A toolbar button with an icon; `on` draws it selected (accent icon on a pale accent tint).
pub fn button(ui: &mut Ui, icon: Icon, tip: &str, on: bool) -> egui::Response {
    let (r, resp) = ui.allocate_exact_size(vec2(30.0, 26.0), Sense::click());
    let v = ui.visuals();
    let fill = if on {
        v.selection.bg_fill.gamma_multiply(0.22)
    } else if resp.hovered() {
        v.widgets.hovered.weak_bg_fill
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(r, CornerRadius::from(theme::radius::S), fill);
    let c = if on { v.selection.bg_fill } else { v.text_color() };
    paint(ui, icon, Rect::from_center_size(r.center(), vec2(18.0, 18.0)), c);
    resp.on_hover_text(tip)
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::pos2;

    #[test]
    fn every_icon_stays_on_its_grid() {
        for icon in Icon::ALL {
            let prims = shapes(icon);
            assert!(!prims.is_empty(), "{icon:?} draws nothing");
            for prim in prims {
                let (x0, y0, x1, y1) = bounds(prim);
                assert!(x0 >= 0.0 && y0 >= 0.0 && x1 <= GRID && y1 <= GRID, "{icon:?}: {prim:?} leaves the 24-unit grid ({x0}, {y0}, {x1}, {y1})");
            }
        }
    }

    #[test]
    fn icons_fill_their_grid_evenly() {
        // A line icon set reads as one family when every icon spans about the same box: none
        // shrinks into a corner or crowds the edge.
        for icon in Icon::ALL {
            let (x0, y0, x1, y1) = shapes(icon)
                .iter()
                .map(bounds)
                .fold((f32::MAX, f32::MAX, f32::MIN, f32::MIN), |a, b| (a.0.min(b.0), a.1.min(b.1), a.2.max(b.2), a.3.max(b.3)));
            let (w, h) = (x1 - x0, y1 - y0);
            assert!(w.max(h) >= 14.0, "{icon:?} is too small: {w} × {h}");
            let (mx, my) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
            assert!((mx - 12.0).abs() <= 2.5 && (my - 12.0).abs() <= 2.5, "{icon:?} is off centre: ({mx}, {my})");
        }
    }

    #[test]
    fn rounded_corners_keep_the_ends_and_stay_inside() {
        let pts = [(0.0, 0.0), (10.0, 0.0), (10.0, 10.0)];
        let open = round_corners(&pts, false, 2.0);
        assert_eq!(open.first(), Some(&(0.0, 0.0)));
        assert_eq!(open.last(), Some(&(10.0, 10.0)));
        assert!(open.iter().all(|&(x, y)| (0.0..=10.0).contains(&x) && (0.0..=10.0).contains(&y)));
        assert!(!open.contains(&(10.0, 0.0)), "the corner itself is cut");
        // A radius larger than the sides is limited to half a side; no corner, no change.
        assert!(round_corners(&pts, true, 100.0).iter().all(|p| p.0.is_finite() && p.1.is_finite()));
        assert_eq!(round_corners(&pts, false, 0.0), pts.to_vec());
        assert_eq!(round_corners(&[(1.0, 1.0), (1.0, 1.0), (1.0, 1.0)], true, 2.0).len(), 15);
    }

    #[test]
    fn the_moon_is_one_closed_crescent() {
        let [Prim::Path(segs, true)] = shapes(Icon::Moon) else {
            panic!("the moon is one closed path");
        };
        let pts = path_points(segs);
        let (Some(first), Some(last)) = (pts.first(), pts.last()) else {
            panic!("no points");
        };
        // The two arcs meet where they should: the path ends where it began.
        assert!((first.0 - last.0).abs() < 0.05 && (first.1 - last.1).abs() < 0.05, "{first:?} … {last:?}");
    }

    #[test]
    fn redo_mirrors_undo() {
        assert_eq!(shapes(Icon::Undo), shapes(Icon::Redo));
        assert!(mirrored(Icon::Redo) && !mirrored(Icon::Undo));
    }

    #[test]
    fn painting_any_icon_at_any_size_never_panics() {
        let ctx = egui::Context::default();
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            for icon in Icon::ALL {
                for side in [0.0, 1.0, 12.0, 18.0, 64.0, f32::NAN] {
                    paint(ui, icon, Rect::from_min_size(pos2(10.0, 10.0), vec2(side, side)), Color32::BLACK);
                }
            }
        });
        // The font atlas upload is not wanted here; egui asserts it is handled before a drop.
        out.textures_delta.clear();
    }
}
