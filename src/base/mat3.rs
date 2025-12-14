use crate::base::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Mat3 {
    pub m: [[f64; 3]; 3],
}

impl Mat3 {
    pub fn new(m: [[f64; 3]; 3]) -> Self {
        Self { m }
    }

    pub fn transpose(&self) -> Mat3 {
        let mut m = [[0.0f64; 3]; 3];

        for i in 0..3 {
            for j in 0..3 {
                m[j][i] = self.m[i][j];
            }
        }

        Mat3::new(m)
    }

    pub fn det(&self) -> f64 {
        let m = &self.m;

        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }

    pub fn inverse(&self) -> Option<Mat3> {
        let det = self.det();

        if det.abs() < 1e-6 {
            return None;
        }

        let m = &self.m;

        let inv = [
            [
                (m[1][1] * m[2][2] - m[1][2] * m[2][1]) / det,
                (m[0][2] * m[2][1] - m[0][1] * m[2][2]) / det,
                (m[0][1] * m[1][2] - m[0][2] * m[1][1]) / det,
            ],
            [
                (m[1][2] * m[2][0] - m[1][0] * m[2][2]) / det,
                (m[0][0] * m[2][2] - m[0][2] * m[2][0]) / det,
                (m[0][2] * m[1][0] - m[0][0] * m[1][2]) / det,
            ],
            [
                (m[1][0] * m[2][1] - m[1][1] * m[2][0]) / det,
                (m[0][1] * m[2][0] - m[0][0] * m[2][1]) / det,
                (m[0][0] * m[1][1] - m[0][1] * m[1][0]) / det,
            ],
        ];

        Some(Mat3::new(inv))
    }
}

impl std::ops::Add for Mat3 {
    type Output = Mat3;
    fn add(self, rhs: Self) -> Self::Output {
        let mut out = self;
        for i in 0..3 {
            for j in 0..3 {
                out.m[i][j] = self.m[i][j] + rhs.m[i][j];
            }
        }
        out
    }
}

impl std::ops::Mul<f64> for Mat3 {
    type Output = Mat3;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut out = self;
        for i in 0..3 {
            for j in 0..3 {
                out.m[i][j] *= rhs;
            }
        }
        out
    }
}

impl std::ops::Mul for &Mat3 {
    type Output = Mat3;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = [[0.0; 3]; 3];

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result[i][j] += self.m[i][k] * rhs.m[k][j];
                }
            }
        }

        Mat3::new(result)
    }
}

impl std::ops::Mul<Vec3> for &Mat3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.m[0][0] * rhs.x + self.m[0][1] * rhs.y + self.m[0][2] * rhs.z,
            self.m[1][0] * rhs.x + self.m[1][1] * rhs.y + self.m[1][2] * rhs.z,
            self.m[2][0] * rhs.x + self.m[2][1] * rhs.y + self.m[2][2] * rhs.z,
        )
    }
}
