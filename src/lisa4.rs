use crate::math::V4D;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Lissajou4D {
    a: f64,
    b: f64,
    c: f64,
    r: f64,
}

#[wasm_bindgen]
impl Lissajou4D {
    #[wasm_bindgen(constructor)]
    pub fn new(a: f64, b: f64, c: f64, r: f64) -> Self {
        Self { a, b, c, r }
    }

    #[wasm_bindgen(getter)]
    pub fn a(&self) -> f64 {
        self.a
    }
    #[wasm_bindgen(getter)]
    pub fn b(&self) -> f64 {
        self.b
    }
    #[wasm_bindgen(getter)]
    pub fn c(&self) -> f64 {
        self.c
    }
    #[wasm_bindgen(getter)]
    pub fn r(&self) -> f64 {
        self.r
    }
}

impl Lissajou4D {
    pub fn position(&self, t: f64) -> V4D {
        let at = self.a * t;
        let bt = self.b * t;
        let ct = self.c * t;

        let sa = at.sin();
        let ca = at.cos();
        let sb = bt.sin();
        let cb = bt.cos();
        let sc = ct.sin();
        let cc = ct.cos();

        self.r * V4D::new(sa * cb * sc, sa * sb * sc, ca * sc, cc)
    }

    pub fn d1(&self, t: f64) -> V4D {
        let a = self.a;
        let b = self.b;
        let c = self.c;

        let at = a * t;
        let bt = b * t;
        let ct = c * t;

        let sa = at.sin();
        let ca = at.cos();
        let sb = bt.sin();
        let cb = bt.cos();
        let sc = ct.sin();
        let cc = ct.cos();

        V4D::new(
            a * ca * cb * sc - b * sa * sb * sc + c * sa * cb * cc,
            a * ca * sb * sc + b * sa * cb * sc + c * sa * sb * cc,
            -a * sa * sc + c * ca * cc,
            -c * sc,
        )
        .normalize()
    }

    pub fn d2(&self, t: f64) -> V4D {
        let a = self.a;
        let b = self.b;
        let c = self.c;

        let at = a * t;
        let bt = b * t;
        let ct = c * t;

        let sa = at.sin();
        let ca = at.cos();
        let sb = bt.sin();
        let cb = bt.cos();
        let sc = ct.sin();
        let cc = ct.cos();

        V4D::new(
            -a * a * sa * cb * sc - 2.0 * a * b * ca * sb * sc - b * b * sa * cb * sc
                + 2.0 * a * c * ca * cb * cc
                - c * c * sa * cb * sc,
            -a * a * sa * sb * sc + 2.0 * a * b * ca * cb * sc - b * b * sa * sb * sc
                + 2.0 * a * c * ca * sb * cc
                - c * c * sa * sb * sc,
            -a * a * ca * sc - 2.0 * a * c * sa * cc - c * c * ca * sc,
            -c * c * cc,
        )
        .normalize()
    }
}
