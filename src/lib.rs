mod vec3;
use std::rc::Rc;

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

mod material;
use material::*;

mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Camera {
    image_height: u32,        // Rendered image height
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Vec3,             // Camera center
    pixel00_loc: Vec3,        // Location of pixel 0, 0
    pixel_delta_u: Vec3,      // Offset to pixel to the right
    pixel_delta_v: Vec3,      // Offset to pixel below
    // Camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
    pub aspect_ratio: f64, // Ratio of image width over height
    pub image_width: u32,  // Rendered image width in pixel count
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    lookfrom: Vec3,
    lookat: Vec3,
    vup: Vec3,
}

#[wasm_bindgen]
impl Camera {
    pub fn new() -> Self {
        Self {
            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Vec3(0.0, 0.0, 0.0),
            pixel00_loc: Vec3(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3(0.0, 0.0, 0.0),
            u: Vec3(0.0, 0.0, 0.0),
            v: Vec3(0.0, 0.0, 0.0),
            w: Vec3(0.0, 0.0, 0.0),
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Vec3(0.0, 0.0, 0.0),
            lookat: Vec3(0.0, 0.0, -1.0),
            vup: Vec3(0.0, 1.0, 0.0),
        }
    }

    pub fn render(&mut self, world: &HittableList) -> Vec<String> {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        let mut img = Vec::new();
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Vec3(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }
                img.push(color::write_color(
                    // Averaging the sample
                    &(self.pixel_samples_scale * pixel_color),
                ));
            }
        }

        eprintln!("\rDone.                 ");
        img
    }

    pub fn set_lookfrom(&mut self, u: f64, v: f64, w: f64) {
        self.lookfrom = Vec3(u, v, w);
    }

    pub fn set_lookat(&mut self, u: f64, v: f64, w: f64) {
        self.lookat = Vec3(u, v, w);
    }

    pub fn set_vup(&mut self, u: f64, v: f64, w: f64) {
        self.vup = Vec3(u, v, w);
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        // Determine viewport dimensions.
        let focal_length = (self.lookfrom - self.lookat).length();
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u, v, w unit basis vectors for the camera coordinate frame
        self.w = Vec3::unit_vector(&(self.lookfrom - self.lookat));
        self.u = Vec3::unit_vector(&Vec3::cross(&self.vup, &self.w));
        self.v = Vec3::cross(&self.w, &self.u);

        // Calculate vectors across viewport edges
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // Calculate delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate location of upper left viewport point and pixel
        let viewport_upper_left =
            self.center - (focal_length * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the origin and directed at a randomly
        // sampled point around the pixel location i, j

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.0) * self.pixel_delta_u)
            + ((j as f64 + offset.1) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }

    fn sample_square() -> Vec3 {
        // < -0.5, 0.5 > square sample region around the pixel (unit square)
        Vec3(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn ray_color(r: &Ray, depth: u32, world: &impl Hittable) -> Vec3 {
        // No more light is gathered when ray bounce limit is exceeded
        if depth <= 0 {
            return Vec3(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord {
            p: Vec3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat: Rc::new(Lambertian::new(&Vec3(0.0, 0.0, 0.0))),
        };

        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::new(&Vec3(0.0, 0.0, 0.0), &Vec3(0.0, 0.0, 0.0));
            let mut attenuation = Vec3(0.0, 0.0, 0.0);
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }
            return Vec3(0.0, 0.0, 0.0);
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

    // pub fn add_matte_sphere(&mut self, x: f64, y: f64, z: f64, r: f64, mat: Rc<dyn Material>) {
    //     self.objects
    //         .push(Box::new(Sphere::new(&Vec3(x, y, z), r, mat.clone())));
    // }
    pub fn add_matte_sphere(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        radius: f64,
        r: f64,
        g: f64,
        b: f64,
    ) {
        self.objects.push(Box::new(Sphere::new(
            &Vec3(x, y, z),
            radius,
            Rc::new(Lambertian::new(&Vec3(r, g, b))),
        )));
    }

    pub fn add_metal_sphere(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        radius: f64,
        r: f64,
        g: f64,
        b: f64,
        fuzz: f64,
    ) {
        self.objects.push(Box::new(Sphere::new(
            &Vec3(x, y, z),
            radius,
            Rc::new(Metal::new(&Vec3(r, g, b), fuzz)),
        )));
    }

    pub fn add_glass_sphere(&mut self, x: f64, y: f64, z: f64, radius: f64, idr: f64) {
        self.objects.push(Box::new(Sphere::new(
            &Vec3(x, y, z),
            radius,
            Rc::new(Dielectric::new(idr)),
        )));
    }

    pub fn add_hollow_glass_sphere(&mut self, x: f64, y: f64, z: f64, radius: f64, thickness: f64) {
        self.objects.push(Box::new(Sphere::new(
            &Vec3(x, y, z),
            radius,
            Rc::new(Dielectric::new(1.5)),
        )));
        self.objects.push(Box::new(Sphere::new(
            &Vec3(x, y, z),
            radius - thickness,
            Rc::new(Dielectric::new(1.0 / 1.5)),
        )));
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
