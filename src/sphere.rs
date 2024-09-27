use nalgebra_glm::{Vec3, dot};
use crate::ray_intersect::{RayIntersect, Intersect};
use crate::material::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    fn get_uv(&self, point: &Vec3) -> (f32, f32) {
        let normalized = (point - self.center) / self.radius;
        let u = 0.5 + (normalized.z.atan2(normalized.x) / (2.0 * std::f32::consts::PI));
        let v = 0.5 - (normalized.y.asin() / std::f32::consts::PI);
        (u, v)
    }

    fn perturb_normal(&self, normal: &Vec3, tangent_normal: &Vec3) -> Vec3 {
        // Create a local coordinate system
        let tangent = if normal.x.abs() > normal.y.abs() {
            Vec3::new(-normal.z, 0.0, normal.x).normalize()
        } else {
            Vec3::new(0.0, -normal.z, normal.y).normalize()
        };
        let bitangent = normal.cross(&tangent);

        // Transform the tangent normal to world space
        let perturbed_normal = tangent * tangent_normal.x + bitangent * tangent_normal.y + normal * tangent_normal.z;

        let blend_factor = 10.0; // Adjust this value to control the strength of the normal map
        (normal * (1.0 - blend_factor) + perturbed_normal * blend_factor).normalize()
    }
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        // Vector from the ray origin to the center of the sphere
        let oc = ray_origin - self.center;

        // Coefficients for the quadratic equation
        let a = dot(ray_direction, ray_direction);
        let b = 2.0 * dot(&oc, ray_direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;

        // Discriminant of the quadratic equation
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > 0.0 {
                let point = ray_origin + ray_direction * t;
                let geometric_normal = (point - self.center).normalize();
                let distance = t; 
                let (u, v) = self.get_uv(&point);

                let normal = if self.material.has_normal_map {
                    let tangent_normal = self.material.get_normal_from_map(u, v);
                    self.perturb_normal(&geometric_normal, &tangent_normal)
                } else {
                    geometric_normal
                };

                return Intersect::new(point, normal, distance, self.material.clone(), u, v);
            }
        }

        // If no intersection, return an empty intersect
        Intersect::empty()
    }
}



