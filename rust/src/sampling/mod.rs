pub mod fixed_count;
pub use fixed_count::*;

use glam::Vec2;
use harfrust::GlyphBuffer;
use skrifa::{FontRef, outline::DrawError};

use crate::{
    curves::{CubicBezier, Line, QuadBezier},
    shaping::{BasePen, PenOutput, draw_glyph_buf},
};

pub fn sample_glyph_buf<'a>(
    sampler: &'a dyn Sampler,
    font_ref: &FontRef,
    buf: GlyphBuffer,
) -> Result<Vec<Vec2>, DrawError> {
    draw_glyph_buf(
        Box::new(PolygonOutput {
            sampler,
            output: Vec::new(),
        }),
        font_ref,
        buf,
    )
}

/// Sample points in the range (0,1]
pub trait Sampler {
    fn sample_cubic(&self, curve: CubicBezier) -> Vec<Vec2>;

    fn sample_quad(&self, curve: QuadBezier) -> Vec<Vec2> {
        self.sample_cubic(CubicBezier::from(curve))
    }

    fn sample_line(&self, line: Line) -> Vec<Vec2> {
        return vec![line.p1];
    }
}

struct PolygonOutput<'a> {
    sampler: &'a dyn Sampler,
    output: Vec<Vec2>,
}

impl<'a> PenOutput for PolygonOutput<'a> {
    type Output = Vec<Vec2>;

    fn output(self: Box<Self>) -> Self::Output {
        self.output
    }

    fn move_to(&mut self, base: &BasePen, p1: Vec2) {
        self.output.push(Vec2::NAN);
        self.output.push(base.glyph_pos() + p1);
    }

    fn line_to(&mut self, base: &BasePen, p1: Vec2) {
        self.output.extend(
            self.sampler
                .sample_line(Line {
                    p0: base.current_point,
                    p1,
                })
                .into_iter()
                .map(|p| base.glyph_pos() + p),
        );
    }

    fn quad_to(&mut self, base: &BasePen, c: Vec2, p1: Vec2) {
        self.output.extend(
            self.sampler
                .sample_quad(QuadBezier {
                    p0: base.current_point,
                    c,
                    p1,
                })
                .into_iter()
                .map(|p| base.glyph_pos() + p),
        );
    }

    fn curve_to(&mut self, base: &BasePen, c0: Vec2, c1: Vec2, p1: Vec2) {
        self.output.extend(
            self.sampler
                .sample_cubic(CubicBezier {
                    p0: base.current_point,
                    c0,
                    c1,
                    p1,
                })
                .into_iter()
                .map(|p| base.glyph_pos() + p),
        );
    }

    fn close(&mut self, base: &BasePen) {
        self.output.push(base.glyph_pos() + base.path_start);
    }
}
