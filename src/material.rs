use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;
use crate::rtweekend::*;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: &Vec3) -> Self {
        Self { albedo: *albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction (rand_unit_vec is -rec.normal)
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(&rec.p, &scatter_direction);
        *attenuation = self.albedo;

        true
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Vec3, fuzz: f64) -> Self {
        Self {
            albedo: *albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(r_in.direction(), &rec.normal);
        let reflected = Vec3::unit_vector(&reflected) + (self.fuzz * Vec3::random_unit_vector());
        *scattered = Ray::new(&rec.p, &reflected);
        *attenuation = self.albedo;

        Vec3::dot(scattered.direction(), &rec.normal) > 0.0
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Schlick's approximation for reflectance
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3(1.0, 1.0, 1.0);
        let ri: f64 = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = Vec3::unit_vector(r_in.direction());

        let cos_theta = Vec3::dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, ri)
        };

        *scattered = Ray::new(&rec.p, &direction);
        true
    }
}
