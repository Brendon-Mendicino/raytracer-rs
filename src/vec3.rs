use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, Neg, Range, Sub, SubAssign},
};

use rand::Rng;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec3(f32, f32, f32);

impl Vec3 {
    pub const ZERO: Self = Self(0.0, 0.0, 0.0);

    pub fn new(v: (f32, f32, f32)) -> Self {
        Self(v.0, v.1, v.2)
    }

    pub fn rand_with_range(range: Range<f32>) -> Vec3 {
        Self(
            rand::thread_rng().gen_range(range.clone()),
            rand::thread_rng().gen_range(range.clone()),
            rand::thread_rng().gen_range(range),
        )
    }

    pub fn rand_unit() -> Vec3 {
        let v = Self::rand_with_range(-1.0..1.0);
        Self::unit(v)
    }

    pub fn scale(a: f32, v: Vec3) -> Vec3 {
        Self(a * v.0, a * v.1, a * v.2)
    }

    pub fn dot(v: Vec3, u: Vec3) -> f32 {
        v.0 * u.0 + v.1 * u.1 + v.2 * u.2
    }

    pub fn norm(v: Vec3) -> f32 {
        f32::sqrt(Self::dot(v, v))
    }

    pub fn unit(v: Vec3) -> Vec3 {
        1.0 / Self::norm(v) * v
    }

    pub fn elem_dot(v: Self, u: Self) -> Self {
        Self(v.0 * u.0, v.1 * u.1, v.2 * u.2)
    }

    #[inline]
    pub fn lambertian_distribution(normal: Self) -> Self {
        normal + Self::rand_unit()
    }

    #[inline]
    pub fn reflect(v: Vec3, n: Vec3) -> Self {
        v - 2.0 * Self::dot(v, n) * n
    }

    pub fn x(self) -> f32 {
        self.0
    }

    pub fn y(self) -> f32 {
        self.1
    }

    pub fn z(self) -> f32 {
        self.2
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self(rhs * self.0, rhs * self.1, rhs * self.2)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

impl Into<(f32, f32, f32)> for Vec3 {
    fn into(self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    rgb: Vec3,
}

impl Color {
    pub const BLACK: Self = Self {
        rgb: Vec3(0.0, 0.0, 0.0),
    };

    pub const WHITE: Self = Self {
        rgb: Vec3(1.0, 1.0, 1.0),
    };

    pub const RED: Self = Self {
        rgb: Vec3(1.0, 0.0, 0.0),
    };

    pub const GREEN: Self = Self {
        rgb: Vec3(0.0, 1.0, 0.0),
    };

    pub const BLUE: Self = Self {
        rgb: Vec3(0.0, 0.0, 1.0),
    };

    pub fn new(rgb: (f32, f32, f32)) -> Self {
        Self {
            rgb: Vec3::new(rgb),
        }
    }

    pub fn blend(a: Self, b: Self) -> Self {
        Self {
            rgb: Vec3::elem_dot(a.rgb, b.rgb),
        }
    }

    fn linera_to_gamma(c: f32) -> f32 {
        f32::sqrt(c)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = Self::linera_to_gamma(self.rgb.0);
        let g = Self::linera_to_gamma(self.rgb.1);
        let b = Self::linera_to_gamma(self.rgb.2);

        // Write the translated [0,255] value of each color component.
        write!(
            f,
            "{} {} {}",
            (255.999 * r) as u8,
            (255.999 * g) as u8,
            (255.999 * b) as u8
        )
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            rgb: self.rgb + rhs.rgb,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color::new((self * rhs.rgb.0, self * rhs.rgb.1, self * rhs.rgb.2))
    }
}
