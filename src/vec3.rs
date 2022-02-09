use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Default)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.0*self.0 + self.1*self.1 + self.2*self.2
    }

    pub fn dot(&self, other: Vec3) -> f64 {
            self.0*other.0
            + self.1*other.1
            + self.2*other.2
    }
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3(
            self.1*other.2 - self.2*other.1,
            self.2*other.0 - self.0*other.2,
            self.0*other.1 - self.1*other.0
        )
    }
    pub fn unit_vector(self) -> Vec3 {
        let length = self.length();
        Vec3(
            self.0/length,
            self.1/length,
            self.2/length
        )
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3(-self.0, -self.1, -self.2)
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
            self.0 += other.0;
            self.1 += other.1;
            self.2 += other.2;
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + self.2)
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0*rhs.0, self.1*rhs.1, self.2*rhs.2)
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self*rhs.0, self*rhs.1, self*rhs.2)
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self(self.0*rhs, self.1*rhs, self.2*rhs)
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        if rhs == 0.0 {
            panic!("Cannot divide by zero-valued `Rational`!");
        }
        Self(self.0/rhs, self.1/rhs, self.2/rhs)
    }
}