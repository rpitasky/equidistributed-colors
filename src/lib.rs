use hsl::HSL;
pub use rgb::RGB8;

use oklab::*;

const PHI: f64 = 0.618033988749895;

#[derive(Copy, Clone)]
/// Iterator over colors which are distributed evenly according to the equidistribution theorem.
/// Attempts to make the next color look as different as possible from all of the previously generated colors. 
/// 
/// Call [`EquiColor::next()`] to produce the next color. 
pub struct EquiColor {
    state: f64,
    s: f32,
    l: f32,
}

impl EquiColor {
    /// Create a color generator with the provided saturation and lightness (in range `[0.0, 1.0)`)
    pub fn new(s: f32, l: f32) -> Self {
        EquiColor {
            state: 0.0,
            s, l,
        }
    }
}

impl Default for EquiColor {
    fn default() -> Self {
        Self::new(1.0, 0.6)
    }
}

impl Iterator for EquiColor {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        let (r, g, b) = HSL {
            h: 360.0*self.state,
            s: self.s as f64,
            l: self.l as f64,
        }.to_rgb();

        self.state = (self.state + PHI) % 1.0;

        let mut oklab = srgb_to_oklab(RGB8 { r, g, b});
        oklab.l = self.l;

        Some(oklab_to_srgb(oklab))
    }
}

#[test]
fn first_three_colors_correct() {
    let mut equicolor = EquiColor::default();
    assert_eq!(equicolor.next(), Some(Rgb { r: 237, g: 22, b: 33 }));
    assert_eq!(equicolor.next(), Some(Rgb { r: 54, g: 115, b: 255 }));
    assert_eq!(equicolor.next(), Some(Rgb { r: 74, g: 152, b: 0 }));
}