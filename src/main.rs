extern crate rand;

use std::{ops, vec};

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    fn random(min: f32, max: f32) -> Vec3 {
        Vec3 {
            x: random_double(min, max),
            y: random_double(min, max),
            z: random_double(min, max),
        }
    }

    fn len2(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn len(&self) -> f32 {
        self.len2().sqrt()
    }

    fn dot(&self, v: Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, f: f32) -> Self {
        Self {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, f: f32) -> Self {
        Self {
            x: self.x / f,
            y: self.y / f,
            z: self.z / f,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
#[derive(Debug)]
struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3::new(0., 0., 0.);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.
                - vertical / 2.
                - Vec3::new(0., 0., focal_length),
        }
    }

    fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            dir: self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        }
    }
}

#[derive(Copy, Clone)]
struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    fn at(&self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }
}

#[derive(Debug, Copy, Clone)]
struct Sphere {
    center: Vec3,
    radius: f32,
}

struct Hit {
    t: f32,
    p: Vec3,
    normal: Vec3,
    front: bool,
}

impl Hit {
    fn new(t: f32, p: Vec3, normal: Vec3, front: bool) -> Hit {
        Hit {
            t,
            p,
            normal,
            front,
        }
    }
}

enum Intersection {
    Missed,
    Hit(Hit),
}

impl Sphere {
    fn hit(&self, ray: &Ray) -> Intersection {
        let oc = ray.origin - self.center;
        let a = ray.dir.dot(ray.dir);
        let half_b = oc.dot(ray.dir);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return Intersection::Missed;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < 0. || f32::INFINITY < root {
            root = (-half_b + sqrtd) / a;
            if root < 0. || f32::INFINITY < root {
                return Intersection::Missed;
            }
        }

        let p = ray.at(root);
        let outward_normal: Vec3 = (p - self.center) / self.radius;
        let front = ray.dir.dot(outward_normal) < 0.;
        let normal = match front {
            true => outward_normal,
            false => -outward_normal,
        };

        Intersection::Hit(Hit::new(root, p, normal, front))
    }
}

fn unit_vector(v: Vec3) -> Vec3 {
    // Implement / operator between Vec3 and i32 (len).
    let len = v.len();
    Vec3 {
        x: v.x / len,
        y: v.y / len,
        z: v.z / len,
    }
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random(-1., 1.);
        match p.len2() >= 1. {
            true => continue,
            false => return p,
        }
    }
}

fn clip(v: f32, min: f32, max: f32) -> f32 {
    match v {
        c if c > max => max,
        c if c < min => min,
        c => c,
    }
}

fn random_double(min: f32, max: f32) -> f32 {
    min + (max - min) * rand::random::<f32>()
}

fn write_color(p: &Vec3, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f32;
    println!(
        "{} {} {}",
        (clip(p.x * scale, 0., 0.999) * 255.) as i32,
        (clip(p.y * scale, 0., 0.999) * 255.) as i32,
        (clip(p.z * scale, 0., 0.999) * 255.) as i32
    )
}

fn ray_color(ray: &Ray, objects: &Vec<Sphere>, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0., 0., 0.);
    }

    let mut tnear = f32::INFINITY;
    let mut hit: Option<Hit> = None;

    for obj in objects {
        // println!("{:?}", obj);
        match obj.hit(ray) {
            Intersection::Missed => continue,
            Intersection::Hit(h) => {
                // eprintln!("t {}", h.t);
                // eprintln!("s {:?}", obj);
                if h.t < tnear {
                    tnear = h.t;
                    hit = Some(h);
                }
            }
        }
    }

    match hit {
        // Object.
        Some(h) => {
            let target = h.p + h.normal + random_in_unit_sphere();
            ray_color(
                &Ray {
                    origin: h.p,
                    dir: target - h.p,
                },
                objects,
                depth - 1,
            ) * 0.5
        }
        // Background.
        None => {
            let unit_direction = unit_vector(ray.dir);
            let t = 0.5 * (unit_direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 300;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 50;
    let max_depth = 50;

    // Camera
    let cam = Camera::new();

    //
    // Objects in scene.
    //
    let s1 = Sphere {
        center: Vec3 {
            x: 9.,
            y: 0.,
            z: -10.,
        },
        radius: 4.,
    };

    let s2 = Sphere {
        center: Vec3 {
            x: 0.,
            y: 0.,
            z: -10.,
        },
        radius: 4.,
    };

    let s3 = Sphere {
        center: Vec3 {
            x: -9.,
            y: 0.0,
            z: -10.,
        },
        radius: 4.,
    };

    let objects: Vec<Sphere> = vec![s1, s2, s3];

    // Render
    let mut image: Vec<Vec3> = vec![];
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut color = Vec3::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + rand::random::<f32>()) / (image_width - 1) as f32;
                let v = (j as f32 + rand::random::<f32>()) / (image_height - 1) as f32;
                let ray = cam.get_ray(u, v);
                color = color + ray_color(&ray, &objects, max_depth);
            }
            image.push(color);
        }
    }

    println!("P3\n{} {}\n255\n", image_width, image_height);
    for p in &image {
        write_color(p, samples_per_pixel);
    }
    eprintln!("Done!");
}
