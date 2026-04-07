mod vec3;
use vec3::{Vec3, color};

mod ray;
use ray::Ray;

mod hittable;
use hittable::*;

mod rtweekend;
use rtweekend::*;

mod shapes;
use crate::shapes::Sphere;

mod interval;
use interval::*;

mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Camera {
    image_height: u16,     // Rendered image height
    center: Vec3,          // Camera center
    pixel00_loc: Vec3,     // Location of pixel 0, 0
    pixel_delta_u: Vec3,   // Offset to pixel to the right
    pixel_delta_v: Vec3,   // Offset to pixel below
    pub aspect_ratio: f64, // Ratio of image width over height
    pub image_width: u16,  // Rendered image width in pixel count
}

#[wasm_bindgen]
impl Camera {
    pub fn new() -> Self {
        Self {
            image_height: 0,
            center: Vec3(0.0, 0.0, 0.0),
            pixel00_loc: Vec3(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3(0.0, 0.0, 0.0),
            aspect_ratio: 1.0,
            image_width: 100,
        }
    }

    pub fn render(&mut self, world: &HittableList) -> Vec<String> {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        let mut img = Vec::new();

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(&self.center, &ray_direction);

                let pixel_color = Self::ray_color(&r, world);
                img.push(color::write_color(&pixel_color));
            }
        }

        eprintln!("\rDone.                 ");
        img
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u16;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = Vec3(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate vectors across viewport edges
        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -viewport_height, 0.0);

        // Calculate delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate location of upper left viewport point and pixel
        let viewport_upper_left =
            self.center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    }

    fn ray_color(r: &Ray, world: &impl Hittable) -> Vec3 {
        // let center = Vec3(0.0, 0.0, -1.0);
        // let radius = 0.5;
        // if let Option::Some(t) = hit_sphere(&center, radius, r) {
        //     let N = Vec3::unit_vector(&(r.at(t) - center));
        //     return Vec3(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0) / 2.0;
        // }
        let mut rec = HitRecord {
            p: Vec3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        };
        if world.hit(r, Interval::new(0.0, INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Vec3(1.0, 1.0, 1.0)); // White color
        }

        let unit_direction = Vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        // linear blend/interpolation (lerp) between white and light blue
        (1.0 - a) * Vec3(1.0, 1.0, 1.0) + a * Vec3(0.5, 0.7, 1.0)
    }
}

#[wasm_bindgen]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

#[wasm_bindgen]
impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    // fn add(&mut self, object: Box<dyn Hittable>) {
    // self.objects.push(object);
    // }

    pub fn add_sphere(&mut self, x: f64, y: f64, z: f64, r: f64) {
        self.objects.push(Box::new(Sphere::new(&Vec3(x, y, z), r)));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}
