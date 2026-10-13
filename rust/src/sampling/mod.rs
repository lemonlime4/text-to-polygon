#![allow(dead_code)]
pub mod control_points;
pub mod subdivision;

#[allow(unused_imports)]
pub use self::{control_points::*, subdivision::*};

use glam::Vec2;
use harfrust::Buffer;
use serde::{Deserialize, Serialize};
use skrifa::{FontRef, outline::DrawError};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    curves::{CubicBezier, Line, QuadBezier},
    shaping::{BasePen, PenOutput, draw_glyph_buf},
    state::FontSettings,
};

#[derive(Tsify, Serialize, Deserialize)]
pub enum SamplingMethod {
    ControlPoints { to_cubic: bool },
    Subdivision { tolerance: f32 },
}

impl SamplingMethod {
    pub fn get_sampler(&self) -> Box<dyn Sampler> {
        match self {
            Self::ControlPoints { to_cubic } => {
                if *to_cubic {
                    Box::new(ControlPointsSampler::<true>)
                } else {
                    Box::new(ControlPointsSampler::<false>)
                }
            }
            Self::Subdivision { tolerance } => Box::new(SubdivisionSampler {
                flatness_tolerance: *tolerance,
            }),
        }
    }
}

pub fn sample_glyph_buf<'a>(
    sampler: &'a dyn Sampler,
    font_ref: &FontRef,
    settings: &FontSettings,
    buf: Buffer,
) -> Result<Vec<Vec2>, DrawError> {
    draw_glyph_buf(
        Box::new(PolygonOutput {
            sampler,
            output: Vec::new(),
        }),
        font_ref,
        settings,
        buf,
    )
}

/// Sample points in the range (0,1]
pub trait Sampler {
    fn sample_cubic(&self, curve: CubicBezier) -> Vec<Vec2>;

    fn sample_quad(&self, curve: QuadBezier) -> Vec<Vec2> {
        self.sample_cubic(curve.into())
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
        if base.current_point != base.path_start {
            self.output.push(base.glyph_pos() + base.path_start);
        }
    }
}
