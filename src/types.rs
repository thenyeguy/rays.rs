use std::fmt;
use std::ops::{Add, Div, Index, Mul, Neg, Sub};

#[derive(Copy, Clone)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Copy, Clone)]
pub struct Point3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Copy, Clone)]
pub struct Mat3 {
    values: [[f32; 3]; 3],
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn norm_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(self) -> f32 {
        self.norm_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.norm()
    }

    pub fn dot(self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(self, incident: Vector3) -> Vector3 {
        incident - 2.0 * self.dot(incident) * self
    }

    pub fn refract(self, incident: Vector3, eta: f32) -> Option<Vector3> {
        let cosi = self.dot(incident).abs();
        let cost2 = 1.0 - eta.powi(2) * (1.0 - cosi.powi(2));
        if cost2 < 0.0 {
            None
        } else {
            Some(eta * incident + (eta * cosi - cost2.sqrt()) * self)
        }
    }

    pub fn tangent_space(self) -> Mat3 {
        let tangent = if self.x.abs() > 0.99 {
            Vector3::new(self.y, -self.x, 0.0)
        } else {
            Vector3::new(0.0, self.z, -self.y)
        };
        let binormal = self.cross(tangent);
        Mat3 {
            values: [
                [tangent.x(), binormal.x(), self.x()],
                [tangent.y(), binormal.y(), self.y()],
                [tangent.z(), binormal.z(), self.z()],
            ],
        }
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Vector3::new(x, y, z)
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("out of bounds for vector"),
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        -1.0 * self
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f32) -> Vector3 {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Vector3 {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point3 { x, y, z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
}

impl Index<usize> for Point3 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("out of bounds for point"),
        }
    }
}

impl From<(f32, f32, f32)> for Point3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Point3::new(x, y, z)
    }
}

impl Add<Vector3> for Point3 {
    type Output = Point3;
    fn add(self, v: Vector3) -> Point3 {
        Point3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Sub<Vector3> for Point3 {
    type Output = Point3;
    fn sub(self, v: Vector3) -> Point3 {
        Point3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Sub<Point3> for Point3 {
    type Output = Vector3;
    fn sub(self, other: Point3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl fmt::Debug for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Mat3 {
    pub fn new(values: [[f32; 3]; 3]) -> Self {
        Mat3 { values }
    }
}

impl Index<(usize, usize)> for Mat3 {
    type Output = f32;
    fn index(&self, (i, j): (usize, usize)) -> &f32 {
        &self.values[i][j]
    }
}

impl Mul for Mat3 {
    type Output = Mat3;
    fn mul(self, other: Mat3) -> Mat3 {
        let mut values = [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    values[i][j] += self[(i, k)] * other[(k, j)];
                }
            }
        }
        Mat3::new(values)
    }
}

impl Mul<Vector3> for Mat3 {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Vector3 {
        Vector3::new(
            self[(0, 0)] * v.x + self[(0, 1)] * v.y + self[(0, 2)] * v.z,
            self[(1, 0)] * v.x + self[(1, 1)] * v.y + self[(1, 2)] * v.z,
            self[(2, 0)] * v.x + self[(2, 1)] * v.y + self[(2, 2)] * v.z,
        )
    }
}

impl fmt::Debug for Mat3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sep = if f.alternate() { "\n  " } else { "; " };
        write!(f, "[")?;
        if f.alternate() {
            write!(f, "{}", sep)?;
        }
        write!(f, "{} {} {}", self[(0, 0)], self[(0, 1)], self[(0, 2)])?;
        write!(f, "{}", sep)?;
        write!(f, "{} {} {}", self[(1, 0)], self[(1, 1)], self[(1, 2)],)?;
        write!(f, "{}", sep)?;
        write!(f, "{} {} {}", self[(2, 0)], self[(2, 1)], self[(2, 2)])?;
        if f.alternate() {
            writeln!(f)?;
        }
        write!(f, "]")
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Index<Axis> for Point3 {
    type Output = f32;
    fn index(&self, axis: Axis) -> &f32 {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}
