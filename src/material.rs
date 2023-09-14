use crate::{
    ray::Ray,
    vec3::{Color, Vec3},
};

#[derive(Debug, Clone, Copy, Default)]
pub enum MaterialType {
    #[default]
    Metal,
    Lambertian,
}

#[derive(Debug, Clone, Copy)]
pub enum Scatter {
    Absorbed { solid_color: Color },
    Scattered { direction: Vec3, attenuation: Color },
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Material {
    pub material: MaterialType,
    pub solid_color: Color,
    pub fuzz: Option<f32>,
}

impl Material {
    pub fn metal(solid_color: Color, fuzz: Option<f32>) -> Self {
        Self {
            material: MaterialType::Metal,
            solid_color,
            fuzz,
        }
    }

    pub fn lambertian(solid_color: Color, fuzz: Option<f32>) -> Self {
        Self {
            material: MaterialType::Lambertian,
            solid_color,
            fuzz,
        }
    }

    pub fn scatter(&self, r: Ray, normal: Vec3) -> Scatter {
        let mut result = match self.material {
            MaterialType::Lambertian => {
                Scatter::Scattered {
                    direction: Vec3::lambertian_distribution(normal),
                    attenuation: self.solid_color,
                }
            }
            MaterialType::Metal => {
                let mut direction = Vec3::reflect(r.dir, normal);
                if Vec3::norm(direction) < 1e-8 {
                    direction = normal;
                }

                Scatter::Scattered {
                    direction,
                    attenuation: self.solid_color,
                }
            }
        };

        result = match (self.fuzz, result) {
            (
                Some(fuzz),
                Scatter::Scattered {
                    direction,
                    attenuation,
                },
            ) => {
                let direction = Vec3::unit(direction) + fuzz * Vec3::rand_unit();

                if Vec3::dot(direction, normal) < 0.0 {
                    Scatter::Absorbed {
                        solid_color: self.solid_color,
                    }
                } else {
                    Scatter::Scattered {
                        direction,
                        attenuation,
                    }
                }
            }
            _ => result,
        };

        result
    }
}
