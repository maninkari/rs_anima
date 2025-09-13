use crate::math::V3D;
use crate::polygon::Polygon3D;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Lissajou3D {
    a: f64,
    b: f64,
    r: f64,
}

#[wasm_bindgen]
impl Lissajou3D {
    #[wasm_bindgen(constructor)]
    pub fn new(a: f64, b: f64, r: f64) -> Self {
        Self { a, b, r }
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
    pub fn r(&self) -> f64 {
        self.r
    }
}

impl Lissajou3D {
    // Lissajous position
    pub fn position(&self, t: f64) -> V3D {
        let at = self.a * t;
        let bt = self.b * t;
        self.r * V3D::new(at.sin() * bt.cos(), at.sin() * bt.sin(), at.cos())
    }

    // Tangent (normalized)
    pub fn d1(&self, t: f64) -> V3D {
        let a = self.a;
        let b = self.b;
        let at = a * t;
        let bt = b * t;

        V3D::new(
            a * at.cos() * bt.cos() - b * at.sin() * bt.sin(),
            a * at.cos() * bt.sin() + b * at.sin() * bt.cos(),
            -a * at.sin(),
        )
        .normalize()
    }

    // Normal (normalized)
    pub fn d2(&self, t: f64) -> V3D {
        let p = self.position(t);
        let d1 = self.d1(t);
        p.cross(&d1).normalize()
    }

    // Binormal
    pub fn d3(&self, t: f64) -> V3D {
        let d1 = self.d1(t);
        let d2 = self.d2(t);
        d1.cross(&d2)
    }

    // Transform matrix from trihedron
    pub fn transform_matrix(&self, t: f64) -> [[f64; 4]; 4] {
        let pos = self.position(t);
        let d1 = self.d1(t);
        let d2 = self.d2(t);
        let d3 = self.d3(t);

        [
            [d2.x, d3.x, d1.x, pos.x],
            [d2.y, d3.y, d1.y, pos.y],
            [d2.z, d3.z, d1.z, pos.z],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }

    // Generate vertex buffer: [longitude_lines, latitude_lines, tunnel_triangles]
    pub fn generate_tunnel_vertices(
        &self,
        polygon_radius: f64,
        polygon_sides: usize,
        num_polygons: usize,
        show_longitude: bool,
        show_latitude: bool,
        show_tunnel: bool,
    ) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
        let polygon = Polygon3D::new(polygon_radius, polygon_sides);
        let mut longitude_verts = Vec::new();
        let mut latitude_verts = Vec::new();
        let mut tunnel_verts = Vec::new();

        // Generate polygons along curve
        let mut all_polygons = Vec::new();
        let total = num_polygons + 1; // Extra polygon to close loop
        for i in 0..total {
            let t = 2.0 * std::f64::consts::PI * (i as f64) / (num_polygons as f64);
            let matrix = self.transform_matrix(t);
            let transformed = polygon.transform(&matrix);
            all_polygons.push(transformed);
        }

        // Generate longitude lines (along curve direction)
        if show_longitude {
            for vertex_idx in 0..polygon_sides {
                for poly_idx in 0..(num_polygons) {
                    let v0 = &all_polygons[poly_idx][vertex_idx];
                    let v1 = &all_polygons[poly_idx + 1][vertex_idx];

                    longitude_verts.extend([v0.x as f32, v0.y as f32, v0.z as f32]);
                    longitude_verts.extend([v1.x as f32, v1.y as f32, v1.z as f32]);
                }
            }
        }

        // Generate latitude lines (around each polygon)
        if show_latitude {
            for poly_idx in 0..num_polygons {
                let poly = &all_polygons[poly_idx];
                for vertex_idx in 0..polygon_sides {
                    let v0 = &poly[vertex_idx];
                    let v1 = &poly[(vertex_idx + 1) % polygon_sides];

                    latitude_verts.extend([v0.x as f32, v0.y as f32, v0.z as f32]);
                    latitude_verts.extend([v1.x as f32, v1.y as f32, v1.z as f32]);
                }
            }
        }

        // Generate tunnel triangles
        if show_tunnel {
            for poly_idx in 0..num_polygons {
                for vertex_idx in 0..polygon_sides {
                    let next_vertex = (vertex_idx + 1) % polygon_sides;

                    let v0 = &all_polygons[poly_idx][vertex_idx];
                    let v1 = &all_polygons[poly_idx + 1][vertex_idx];
                    let v2 = &all_polygons[poly_idx][next_vertex];
                    let v3 = &all_polygons[poly_idx + 1][next_vertex];

                    // Calculate quad center and curve position to determine outward direction
                    let t_current = 2.0 * std::f64::consts::PI * (poly_idx as f64)
                        / ((all_polygons.len() - 1) as f64);
                    let curve_center = self.position(t_current);

                    let quad_center = V3D::new(
                        (v0.x + v1.x + v2.x + v3.x) / 4.0,
                        (v0.y + v1.y + v2.y + v3.y) / 4.0,
                        (v0.z + v1.z + v2.z + v3.z) / 4.0,
                    );

                    // Vector from curve center to quad center (outward direction)
                    let outward = V3D::new(
                        quad_center.x - curve_center.x,
                        quad_center.y - curve_center.y,
                        quad_center.z - curve_center.z,
                    );

                    // Calculate normal for triangle v0, v1, v2
                    let edge1 = V3D::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
                    let edge2 = V3D::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
                    let normal = edge1.cross(&edge2);

                    // Check if normal points outward (positive dot product)
                    let faces_outward = normal.dot(&outward) > 0.0;

                    if faces_outward {
                        // Normal already points outward, use counter-clockwise winding
                        // Triangle 1: v0, v1, v2
                        tunnel_verts.extend([v0.x as f32, v0.y as f32, v0.z as f32]);
                        tunnel_verts.extend([v1.x as f32, v1.y as f32, v1.z as f32]);
                        tunnel_verts.extend([v2.x as f32, v2.y as f32, v2.z as f32]);

                        // Triangle 2: v1, v3, v2
                        tunnel_verts.extend([v1.x as f32, v1.y as f32, v1.z as f32]);
                        tunnel_verts.extend([v3.x as f32, v3.y as f32, v3.z as f32]);
                        tunnel_verts.extend([v2.x as f32, v2.y as f32, v2.z as f32]);
                    } else {
                        // Normal points inward, reverse winding to make it point outward
                        // Triangle 1: v0, v2, v1 (reversed)
                        tunnel_verts.extend([v0.x as f32, v0.y as f32, v0.z as f32]);
                        tunnel_verts.extend([v2.x as f32, v2.y as f32, v2.z as f32]);
                        tunnel_verts.extend([v1.x as f32, v1.y as f32, v1.z as f32]);

                        // Triangle 2: v1, v2, v3 (reversed)
                        tunnel_verts.extend([v1.x as f32, v1.y as f32, v1.z as f32]);
                        tunnel_verts.extend([v2.x as f32, v2.y as f32, v2.z as f32]);
                        tunnel_verts.extend([v3.x as f32, v3.y as f32, v3.z as f32]);
                    }
                }
            }
        }

        (longitude_verts, latitude_verts, tunnel_verts)
    }
}

impl std::ops::Mul<V3D> for f64 {
    type Output = V3D;

    fn mul(self, rhs: V3D) -> V3D {
        V3D {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}
