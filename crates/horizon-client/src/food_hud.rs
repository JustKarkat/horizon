//! The food details extra (`foodDetails`): three overlays on the vanilla
//! hunger row, drawn after `hud_layout`'s food icons so they read on top of
//! them. The row's geometry is vanilla's: ten icons from `w / 2 + 91`
//! rightward, each 8 GUI pixels apart with a 9x9 sprite, on `h - 39`. The
//! ideas are ported, not a mod's code; nothing here runs while the extra or
//! its part is off.

use crate::hud_layout::{Blend, GuiDraw, ceil, fill};

/// The vanilla hunger row: the leftmost icon's x and the row's top y. The
/// icons sit at `w / 2 + 91 - n * 8 - 9`, so the row spans 81 pixels.
pub fn row(w: i32, h: i32) -> (i32, i32) {
    (w / 2 + 91 - 81, h - 39)
}

/// The hunger row's overlays for one frame. `food` is the vanilla bar's
/// points (0..=20), `saturation` and `exhaustion` the player's FoodData.
/// `held` is the held food's `minecraft:food` component: its nutrition and
/// `saturation` modifier (`FoodData.eat` restores
/// `nutrition * modifier * 2` saturation, capped at the new food).
pub struct Inputs {
    pub w: i32,
    pub h: i32,
    pub food: i32,
    pub saturation: f32,
    pub exhaustion: f32,
    /// The saturation part, on: a golden ghost over the bar's first points.
    pub saturation_on: bool,
    /// The exhaustion part, on: a dark strip growing across the row.
    pub exhaustion_on: bool,
    /// The held food part, on: ghosts after the bar's points for what eating
    /// the held food would restore.
    pub values_on: bool,
    pub held: Option<(i32, f32)>,
}

/// A semi-transparent sprite draw: the food icon ghosted in a tint.
fn ghost(sprite: &'static str, x: i32, y: i32, color: u32) -> GuiDraw {
    GuiDraw::Sprite {
        sprite: std::borrow::Cow::Borrowed(sprite),
        x,
        y,
        w: 9,
        h: 9,
        region: None,
        color,
        blend: Blend::Textured,
    }
}

/// The tint the saturation ghost draws with: half-gold.
pub const SATURATION_TINT: u32 = 0x80FF_D860;
/// The tint the held food's hunger ghost draws with: half pale white.
pub const HUNGER_TINT: u32 = 0x80FF_FFFF;
/// The tint the held food's saturation ghost draws with over it.
pub const HELD_SATURATION_TINT: u32 = 0xB0FF_D860;
/// The exhaustion strip: a dark translucent cover.
pub const EXHAUSTION_COLOR: u32 = 0x6600_0000;
/// `FoodData` exhausts at 4.0.
pub const EXHAUSTION_LIMIT: f32 = 4.0;

pub fn layout(i: &Inputs) -> Vec<GuiDraw> {
    let mut out = Vec::new();
    if !i.saturation_on && !i.exhaustion_on && !i.values_on {
        return out;
    }
    let (x0, y) = row(i.w, i.h);
    if i.saturation_on {
        let points = ceil(i.saturation.clamp(0.0, i.food as f32));
        // The bar fills from the right (vanilla's food order); an icon holds
        // points 2n+1 and 2n+2.
        for n in 0..(points + 1) / 2 {
            let x = x0 + (10 - 1 - n) * 8;
            let sprite = if n * 2 + 2 <= points {
                "hud/food_full"
            } else {
                "hud/food_half"
            };
            out.push(ghost(sprite, x, y, SATURATION_TINT));
        }
    }
    if i.exhaustion_on {
        let part = (i.exhaustion / EXHAUSTION_LIMIT).clamp(0.0, 1.0);
        let width = (81.0 * part).round() as i32;
        if width > 0 {
            out.push(fill(x0, y, x0 + width, y + 9, EXHAUSTION_COLOR));
        }
    }
    if i.values_on
        && let Some((nutrition, modifier)) = i.held
        && nutrition > 0
    {
        // What eating the held food would restore, ghosted after the bar's
        // own points. `held` carries the `minecraft:food` component's
        // nutrition and `saturation` modifier; `FoodData.eat` restores
        // `nutrition * modifier * 2` saturation, capped at the new food.
        let from = i.food.clamp(0, 20);
        let to = (from + nutrition).min(20);
        let gained_saturation = (nutrition as f32 * modifier * 2.0).min((to - from) as f32);
        let saturation_points = ceil(gained_saturation);
        // The bar's icons hold points 2n+1 and 2n+2, counted from the right.
        for n in 0..10 {
            let covers = |p: i32, end: i32| p > from && p <= end;
            let (lo, hi) = (n * 2 + 1, n * 2 + 2);
            let x = x0 + (9 - n) * 8;
            if covers(lo, to) || covers(hi, to) {
                let sprite = if covers(lo, to) && covers(hi, to) {
                    "hud/food_full"
                } else {
                    "hud/food_half"
                };
                out.push(ghost(sprite, x, y, HUNGER_TINT));
                // The saturation it would restore, gold over the same icons.
                let sat_end = from + saturation_points;
                if covers(lo, sat_end) || covers(hi, sat_end) {
                    let sprite = if covers(lo, sat_end) && covers(hi, sat_end) {
                        "hud/food_full"
                    } else {
                        "hud/food_half"
                    };
                    out.push(ghost(sprite, x, y, HELD_SATURATION_TINT));
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> Inputs {
        Inputs {
            w: 400,
            h: 300,
            food: 20,
            saturation: 5.0,
            exhaustion: 0.0,
            saturation_on: false,
            exhaustion_on: false,
            values_on: false,
            held: None,
        }
    }

    fn draws(i: &Inputs) -> Vec<(String, i32, i32, u32)> {
        layout(i)
            .into_iter()
            .map(|d| match d {
                GuiDraw::Sprite {
                    sprite,
                    x,
                    y,
                    color,
                    ..
                } => (sprite.to_string(), x, y, color),
                GuiDraw::Fill {
                    x0,
                    y0,
                    x1,
                    y1,
                    color,
                } => (format!("fill {} {}", x1 - x0, y1 - y0), x0, y0, color),
                _ => unreachable!("food_hud draws sprites and fills only"),
            })
            .collect()
    }

    #[test]
    fn off_by_default_draws_nothing() {
        assert!(layout(&inputs()).is_empty());
    }

    #[test]
    fn saturation_ghosts_the_bar_from_the_right() {
        let mut i = inputs();
        i.saturation_on = true;
        i.saturation = 5.0;
        let d = draws(&i);
        // Five points: two full icons and a half, rightmost first (x0 = 210).
        assert_eq!(d.len(), 3);
        assert_eq!(
            d[0],
            ("hud/food_full".into(), 210 + 8 * 9, 261, SATURATION_TINT)
        );
        assert_eq!(
            d[1],
            ("hud/food_full".into(), 210 + 8 * 8, 261, SATURATION_TINT)
        );
        assert_eq!(
            d[2],
            ("hud/food_half".into(), 210 + 8 * 7, 261, SATURATION_TINT)
        );
        // Saturation never paints past the food bar.
        i.saturation = 99.0;
        assert_eq!(draws(&i).len(), 10);
    }

    #[test]
    fn the_exhaustion_strip_grows_across_the_row() {
        let mut i = inputs();
        i.exhaustion_on = true;
        i.exhaustion = 2.0;
        assert_eq!(
            draws(&i),
            vec![("fill 41 9".into(), 210, 261, EXHAUSTION_COLOR)]
        );
        i.exhaustion = 8.0;
        let (name, _, _, _) = draws(&i)[0].clone();
        assert_eq!(name, "fill 81 9");
    }

    #[test]
    fn held_food_ghosts_what_it_would_restore() {
        let mut i = inputs();
        i.values_on = true;
        i.food = 12;
        // An apple: 4 hunger (2 icons), saturation 0.3 -> 4 * 2 * 0.3 = 2.4.
        i.held = Some((4, 0.3));
        let d = draws(&i);
        // Two full hunger ghosts at icons 6 and 7 (points 13..16), then the
        // 2.4 points of gold saturation over them: a full and a half.
        let hunger: Vec<_> = d
            .iter()
            .filter(|(_, _, _, c)| *c == HUNGER_TINT)
            .cloned()
            .collect();
        assert_eq!(
            hunger,
            vec![
                ("hud/food_full".to_owned(), 210 + 8 * 3, 261, HUNGER_TINT),
                ("hud/food_full".to_owned(), 210 + 8 * 2, 261, HUNGER_TINT)
            ]
        );
        let gold: Vec<_> = d
            .iter()
            .filter(|(_, _, _, c)| *c == HELD_SATURATION_TINT)
            .cloned()
            .collect();
        assert_eq!(
            gold,
            &[
                (
                    "hud/food_full".into(),
                    210 + 8 * 3,
                    261,
                    HELD_SATURATION_TINT
                ),
                (
                    "hud/food_half".into(),
                    210 + 8 * 2,
                    261,
                    HELD_SATURATION_TINT
                )
            ]
        );
        // Never past the bar's end, and the saturation caps at the hunger.
        i.food = 18;
        i.held = Some((9, 0.8));
        let d = draws(&i);
        assert_eq!(
            d.iter().filter(|(_, _, _, c)| *c == HUNGER_TINT).count(),
            1,
            "{d:?}"
        );
        assert_eq!(
            d.iter()
                .filter(|(_, _, _, c)| *c == HELD_SATURATION_TINT)
                .count(),
            1
        );
    }
}
