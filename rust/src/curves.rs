use glam::Vec2;

pub trait Curve {
    fn eval(&self, t: f32) -> Vec2;
}

pub struct CubicBezier {
    pub p0: Vec2,
    pub c0: Vec2,
    pub c1: Vec2,
    pub p1: Vec2,
}

pub struct QuadBezier {
    pub p0: Vec2,
    pub c: Vec2,
    pub p1: Vec2,
}

pub struct Line {
    pub p0: Vec2,
    pub p1: Vec2,
}

impl Curve for CubicBezier {
    fn eval(&self, t: f32) -> Vec2 {
        let a0 = self.p0.lerp(self.c0, t);
        let a1 = self.c0.lerp(self.c1, t);
        let a2 = self.c1.lerp(self.p1, t);
        let b0 = a0.lerp(a1, t);
        let b1 = a1.lerp(a2, t);
        b0.lerp(b1, t)
    }
}

impl Curve for QuadBezier {
    fn eval(&self, t: f32) -> Vec2 {
        let a0 = self.p0.lerp(self.c, t);
        let a1 = self.c.lerp(self.p1, t);
        a0.lerp(a1, t)
    }
}

impl From<QuadBezier> for CubicBezier {
    fn from(QuadBezier { p0, c, p1 }: QuadBezier) -> Self {
        CubicBezier {
            p0,
            c0: p0.lerp(c, 2. / 3.),
            c1: p1.lerp(c, 2. / 3.),
            p1,
        }
    }
}

impl Curve for Line {
    fn eval(&self, t: f32) -> Vec2 {
        self.p0.lerp(self.p1, t)
    }
}
