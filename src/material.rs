use nalgebra_glm::Vec3;
use std::sync::Arc;

use crate::color::Color;
use crate::texture::Texture;

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 4],
    pub refractive_index: f32,
    pub has_texture: bool,
    pub has_normal_map: bool,
    pub texture: Option<Arc<Texture>>,     
    pub normal_map: Option<Arc<Texture>>,   
}

impl Material {
    pub fn new(diffuse: Color, specular: f32, albedo: [f32; 4], refractive_index: f32) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            refractive_index,
            has_texture: false,
            has_normal_map: false,
            texture: None,
            normal_map: None,
        }
    }

    pub fn new_with_texture(
        specular: f32,
        albedo: [f32; 4],
        refractive_index: f32,
        texture: Arc<Texture>,
        normal_map: Option<Arc<Texture>>, 
    ) -> Self {
        Material {
            diffuse: Color::new(0, 0, 0), 
            specular,
            albedo,
            refractive_index,
            has_texture: true,
            has_normal_map: normal_map.is_some(),
            texture: Some(texture), 
            normal_map,             
        }
    }

    pub fn get_diffuse_color(&self, u: f32, v: f32) -> Color {
        if let Some(texture) = &self.texture {
            texture.get_color(u, v)
        } else {
            self.diffuse
        }
    }

    pub fn get_normal_from_map(&self, u: f32, v: f32) -> Vec3 {
        if let Some(normal_map) = &self.normal_map {
            let color = normal_map.get_color(u, v);

            
            let nx = (color.r as f32 / 255.0) * 2.0 - 1.0;
            let ny = (color.g as f32 / 255.0) * 2.0 - 1.0;
            let nz = color.b as f32 / 255.0; 

            Vec3::new(nx, ny, nz).normalize()
        } else {
            Vec3::new(0.0, 0.0, 1.0) 
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0),
            specular: 0.0,
            albedo: [0.0, 0.0, 0.0, 0.0],
            refractive_index: 0.0,
            has_texture: false,
            has_normal_map: false,
            texture: None,
            normal_map: None,
        }
    }
}
