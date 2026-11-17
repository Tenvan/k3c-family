//! 2×3-Matrix wie `ebiten.GeoM`: `x' = a·x + b·y + tx`, `y' = c·x + d·y + ty`. Jede Operation hängt hinten an
//! (wirkt nach den bisherigen), genau wie bei GeoM.

/// Affine Abbildung in der Reihenfolge von `ebiten.GeoM`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Affine {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub tx: f32,
    pub ty: f32,
}

impl Default for Affine {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Affine {
    pub const IDENTITY: Affine = Affine {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        tx: 0.0,
        ty: 0.0,
    };

    /// Verschiebt nach den bisherigen Operationen (`GeoM.Translate`).
    pub fn translate(&mut self, x: f32, y: f32) -> &mut Self {
        self.tx += x;
        self.ty += y;
        self
    }

    /// Skaliert nach den bisherigen Operationen (`GeoM.Scale`).
    pub fn scale(&mut self, x: f32, y: f32) -> &mut Self {
        self.concat(&Affine {
            a: x,
            d: y,
            ..Self::IDENTITY
        })
    }

    /// Dreht um den Ursprung, Winkel im Bogenmaß, im Uhrzeigersinn auf dem Bildschirm (`GeoM.Rotate`).
    pub fn rotate(&mut self, theta: f32) -> &mut Self {
        let (sin, cos) = theta.sin_cos();
        self.concat(&Affine {
            a: cos,
            b: -sin,
            c: sin,
            d: cos,
            tx: 0.0,
            ty: 0.0,
        })
    }

    /// Hängt `o` hinten an: erst `self`, dann `o` (`GeoM.Concat`).
    pub fn concat(&mut self, o: &Affine) -> &mut Self {
        *self = Affine {
            a: o.a * self.a + o.b * self.c,
            b: o.a * self.b + o.b * self.d,
            c: o.c * self.a + o.d * self.c,
            d: o.c * self.b + o.d * self.d,
            tx: o.a * self.tx + o.b * self.ty + o.tx,
            ty: o.c * self.tx + o.d * self.ty + o.ty,
        };
        self
    }

    /// Bildet den Punkt `(x, y)` ab (`GeoM.Apply`).
    pub fn apply(&self, x: f32, y: f32) -> (f32, f32) {
        (self.a * x + self.b * y + self.tx, self.c * x + self.d * y + self.ty)
    }
}

#[cfg(test)]
#[path = "affine_tests.rs"]
mod tests;
