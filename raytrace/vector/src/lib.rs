use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    // constructors
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn from_val(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn from_val1(val: f64) -> Self {
        Self { x: val, y: val, z: val }
    }

    // getters
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    // arithmetic operations
    pub fn norm(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.norm()
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self { x: self.y * rhs.z - self.z * rhs.y, y: self.z * rhs.x - self.x * rhs.z, z: self.x * rhs.y - self.y * rhs.x }
    }

    // color operations
    pub fn clamp(&self) -> Self {
        Self { x: clamp_val(self.x), y: clamp_val(self.y), z: clamp_val(self.z) }
    }

    pub fn as_bytes(&self, alpha: u8) -> [u8; 4] {
        [gamma_correct(self.x), gamma_correct(self.y), gamma_correct(self.z), alpha]
    }
}

// utility functions
fn clamp_val(val: f64) -> f64 {
    if val < 0.0 { 0.0 }
    else if val > 1.0 { 1.0 }
    else { val }
}

fn gamma_correct(val: f64) -> u8 {
    (val.powf(1.0 / 2.2) * 255.0 + 0.5) as u8
}

// operator overloads
impl ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl ops::Div<Vector> for f64 {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Self::Output {
        rhs / self
    }
}

pub type Point = Vector;
pub type Color = Vector;
