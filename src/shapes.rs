use crate::hittable::*;
use crate::interval::*;
use crate::ray::*;
use crate::vec3::*;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f64) -> Self {
        Self {
            center: *center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = Vec3::dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range. note pos. t values from camera
        let mut root = (h - sqrtd) / a; // Check the closer root
        // after this we are checking the ray hitting the "inside" of the sphere?
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a; // Check the farther root
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius; // unit vector for sphere doesn't req. sqrt
        rec.set_face_normal(r, &outward_normal);

        true
    }
}
