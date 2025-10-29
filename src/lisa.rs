use crate::math::V3D;
use crate::polygon::Polygon3D;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Lissajou3D {
    a: f64,
    b: f64,
    r: f64,
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: [f32; 4],
    // pub uv: [f32; 2],
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<u32>,
    pub long_lines: Vec<u32>,
    pub lat_lines: Vec<u32>,
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

    // Normal (radial frame - points outward from origin)
    // This creates a twist-free frame suitable for visualization
    pub fn d2(&self, t: f64) -> V3D {
        let p = self.position(t);
        let d1 = self.d1(t);

        // Use radial direction from origin, make it orthogonal to tangent
        let radial = p.normalize();
        let tangent = d1;

        // Project radial onto plane perpendicular to tangent
        let dot = radial.dot(&tangent);
        let projection = V3D::new(
            radial.x - dot * tangent.x,
            radial.y - dot * tangent.y,
            radial.z - dot * tangent.z,
        );

        projection.normalize()
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

    pub fn generate_tunnel_mesh(
        &self,
        polygon_radius: f64,
        polygon_sides: usize,
        num_polygons: usize,
    ) -> Mesh {
        let polygon = Polygon3D::new(polygon_radius, polygon_sides);
        let rings = num_polygons + 1;
        let mut all_polygons = Vec::new();

        for i in 0..rings {
            let t = 2.0 * std::f64::consts::PI * (i as f64) / (num_polygons as f64);
            let matrix = self.transform_matrix(t);
            all_polygons.push(polygon.transform(&matrix));
        }

        // Create vertices for all rings including the closing ring
        let mut vertices = Vec::with_capacity(rings * polygon_sides);

        // Pre-compute the first ring's color to reuse for the last ring
        let first_rgb = hsv_to_rgb(0.0, 1.0, 1.0);

        for i in 0..rings {
            // Generate color based on position along curve
            // The last ring must use the exact same color as the first ring
            let rgb = if i == num_polygons {
                first_rgb // Reuse first color for perfect loop closure
            } else {
                let delta = (i as f32 / num_polygons as f32) * 2.0 * std::f64::consts::PI as f32;
                (
                    0.5 + 0.5 * delta.sin(),
                    0.35 + 0.35 * (3.0 * delta).cos(),
                    0.75 + 0.25 * (4.0 * delta).sin(),
                )
            };

            for j in 0..polygon_sides {
                let p = &all_polygons[i][j];
                vertices.push(Vertex {
                    pos: [p.x as f32, p.y as f32, p.z as f32],
                    color: [rgb.0, rgb.1, rgb.2, 0.5], // More opaque, less washed out
                });
            }
        }

        // Triangles - connect each ring to the next (last ring connects to ring 0)
        let mut triangles = Vec::new();
        for i in 0..num_polygons {
            for j in 0..polygon_sides {
                let next_side = (j + 1) % polygon_sides;

                let a = (i * polygon_sides + j) as u32;
                let b = ((i + 1) * polygon_sides + j) as u32;
                let c = (i * polygon_sides + next_side) as u32;
                let d = ((i + 1) * polygon_sides + next_side) as u32;
                triangles.extend_from_slice(&[a, b, c, b, d, c]);
            }
        }

        // Longitude lines - along the curve
        let mut long_lines = Vec::new();
        for j in 0..polygon_sides {
            for i in 0..num_polygons {
                let a = (i * polygon_sides + j) as u32;
                let b = ((i + 1) * polygon_sides + j) as u32;
                long_lines.extend_from_slice(&[a, b]);
            }
        }

        // Latitude lines - around each ring
        let mut lat_lines = Vec::new();
        for i in 0..rings {
            for j in 0..polygon_sides {
                let next_side = (j + 1) % polygon_sides;
                let a = (i * polygon_sides + j) as u32;
                let b = (i * polygon_sides + next_side) as u32;
                lat_lines.extend_from_slice(&[a, b]);
            }
        }

        Mesh {
            vertices,
            triangles,
            long_lines,
            lat_lines,
        }
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
