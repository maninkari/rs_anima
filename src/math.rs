#[derive(Clone, Copy, Debug)]
pub struct V3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl V3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            Self {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        } else {
            *self
        }
    }

    pub fn cross(&self, other: &V3D) -> V3D {
        V3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(&self, other: &V3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Apply 4x4 transformation matrix to this vector (treated as point)
    pub fn transform(&self, matrix: &[[f64; 4]; 4]) -> V3D {
        let x =
            matrix[0][0] * self.x + matrix[0][1] * self.y + matrix[0][2] * self.z + matrix[0][3];
        let y =
            matrix[1][0] * self.x + matrix[1][1] * self.y + matrix[1][2] * self.z + matrix[1][3];
        let z =
            matrix[2][0] * self.x + matrix[2][1] * self.y + matrix[2][2] * self.z + matrix[2][3];

        V3D::new(x, y, z)
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

#[derive(Clone, Copy, Debug)]
pub struct V4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl V4D {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            Self {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
                w: self.w / mag,
            }
        } else {
            *self
        }
    }

    pub fn dot(&self, other: &V4D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Project 4D point to 3D using stereographic projection
    pub fn project_to_3d(&self) -> V3D {
        let scale = 1.0 / (1.0 - self.w);
        V3D::new(self.x * scale, self.y * scale, self.z * scale)
    }
}

impl std::ops::Mul<V4D> for f64 {
    type Output = V4D;

    fn mul(self, rhs: V4D) -> V4D {
        V4D {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}
