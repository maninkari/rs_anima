use crate::math::V3D;

#[derive(Clone, Debug)]
pub struct Polygon3D {
    radius: f64,
    sides: usize,
    vertices: Vec<V3D>,
}

impl Polygon3D {
    pub fn new(radius: f64, sides: usize) -> Self {
        let mut vertices = Vec::with_capacity(sides);
        let angle_step = 2.0 * std::f64::consts::PI / (sides as f64);

        for i in 0..sides {
            let angle = (i as f64) * angle_step;
            vertices.push(V3D::new(
                radius * angle.cos(),
                radius * angle.sin(),
                0.0, // Polygon is in XY plane initially
            ));
        }

        Self {
            radius,
            sides,
            vertices,
        }
    }

    /// Transform polygon vertices using a 4x4 transformation matrix
    pub fn transform(&self, matrix: &[[f64; 4]; 4]) -> Vec<V3D> {
        self.vertices.iter().map(|v| v.transform(matrix)).collect()
    }

    /// Generate line vertices for rendering the polygon outline
    pub fn generate_line_vertices(&self, matrix: &[[f64; 4]; 4]) -> Vec<f32> {
        let transformed = self.transform(matrix);
        let mut vertices = Vec::with_capacity(transformed.len() * 6); // 2 vertices per edge

        for i in 0..transformed.len() {
            let current = &transformed[i];
            let next = &transformed[(i + 1) % transformed.len()];

            // Add line segment from current to next vertex
            vertices.push(current.x as f32);
            vertices.push(current.y as f32);
            vertices.push(current.z as f32);
            vertices.push(next.x as f32);
            vertices.push(next.y as f32);
            vertices.push(next.z as f32);
        }

        vertices
    }
}
