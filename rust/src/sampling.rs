use glam::Vec2;

pub trait Sampler {
    /// Sample points in the range (0,1]
    fn sample(&self, curve: CubicBezier) -> Vec<Vec2>;
}

pub struct CubicBezier {
    pub p0: Vec2,
    pub c0: Vec2,
    pub c1: Vec2,
    pub p1: Vec2,
}

impl CubicBezier {
    pub fn eval(&self, t: f32) -> Vec2 {
        let u = 1.0 - t;
        let a0 = self.p0 * u + self.c0 * t;
        let a1 = self.c0 * u + self.c1 * t;
        let a2 = self.c1 * u + self.p1 * t;
        let b0 = a0 * u + a1 * t;
        let b1 = a1 * u + a2 * t;
        b0 * u + b1 * t
    }
}

pub struct FixedCountSampler(pub u32);

impl Sampler for FixedCountSampler {
    fn sample(&self, curve: CubicBezier) -> Vec<Vec2> {
        let n = self.0;
        (1..=n)
            .map(move |i| {
                let t = i as f32 / n as f32;
                curve.eval(t)
            })
            .collect()
    }
}
