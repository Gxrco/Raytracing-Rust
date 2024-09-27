use nalgebra_glm::Vec3;
use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let inv_dir = Vec3::new(1.0 / ray_direction.x, 1.0 / ray_direction.y, 1.0 / ray_direction.z);
        
        let mut tmin = (self.min.x - ray_origin.x) * inv_dir.x;
        let mut tmax = (self.max.x - ray_origin.x) * inv_dir.x;
        if tmin > tmax { std::mem::swap(&mut tmin, &mut tmax); }

        let mut tymin = (self.min.y - ray_origin.y) * inv_dir.y;
        let mut tymax = (self.max.y - ray_origin.y) * inv_dir.y;
        if tymin > tymax { std::mem::swap(&mut tymin, &mut tymax); }

        if (tmin > tymax) || (tymin > tmax) {
            return Intersect::empty();
        }

        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (self.min.z - ray_origin.z) * inv_dir.z;
        let mut tzmax = (self.max.z - ray_origin.z) * inv_dir.z;
        if tzmin > tzmax { std::mem::swap(&mut tzmin, &mut tzmax); }

        if (tmin > tzmax) || (tzmin > tmax) {
            return Intersect::empty();
        }

        if tzmin > tmin {
            tmin = tzmin;
        }
        if tzmax < tmax {
            tmax = tzmax;
        }

        // Now we know we have an intersection
        let intersect_point = ray_origin + ray_direction * tmin;
        let mut normal = Vec3::new(0.0, 0.0, 0.0);
        
        // Determine which face of the cube was hit (determine the normal)
        if tmin == tzmin {
            normal.z = if ray_direction.z < 0.0 { 1.0 } else { -1.0 };
        } else if tmin == tymin {
            normal.y = if ray_direction.y < 0.0 { 1.0 } else { -1.0 };
        } else {
            normal.x = if ray_direction.x < 0.0 { 1.0 } else { -1.0 };
        }

        Intersect::new(intersect_point, normal, tmin, self.material.clone(), 0.0, 0.0)
    }
}
