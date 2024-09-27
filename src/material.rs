use once_cell::sync::Lazy;
use std::sync::Arc;
use nalgebra_glm::Vec3;

use crate::color::Color;
use crate::texture::Texture;

// static BALL: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/ball.png")));
// static BALL_NORMAL: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/ball_normal.png")));


static BALL: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/bricks.png")));
static BALL_NORMAL: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/bricks_normal.png")));

#[derive(Debug, Clone)]
pub struct Material {
  pub diffuse: Color,
  pub specular: f32,
  pub albedo: [f32; 4],
  pub refractive_index: f32,
  pub has_texture: bool,
  pub has_normal_map: bool,
}

impl Material {
  pub fn new(
    diffuse: Color,
    specular: f32,
    albedo: [f32; 4],
    refractive_index: f32,
  ) -> Self {
    Material {
      diffuse,
      specular,
      albedo,
      refractive_index,
      has_texture: false,
      has_normal_map: false,
    }
  }

  pub fn new_with_texture(
    specular: f32,
    albedo: [f32; 4],
    refractive_index: f32,
  ) -> Self {
    Material {
      diffuse: Color::new(0, 0, 0), // Default color, will be overridden by texture
      specular,
      albedo,
      refractive_index,
      has_texture: true,
      has_normal_map: true,
    }
  }

  pub fn get_diffuse_color(&mut self, u: f32, v: f32) -> Color {
    if self.has_texture {
      let x = (u * (BALL.width as f32 - 1.0)) as usize;
      let y = ((1.0 - v) * (BALL.height as f32 - 1.0)) as usize;
      BALL.get_color(x, y)
      // Color::new(255, 0, 0)
    }
    else {
      self.diffuse
    }
  }

  pub fn get_normal_from_map(&self, u: f32, v: f32) -> Vec3 {
    if self.has_normal_map {
      let x = (u * (BALL_NORMAL.width as f32 - 1.0)) as usize;
      let y = ((1.0 - v) * (BALL_NORMAL.height as f32 - 1.0)) as usize;
      let color = BALL_NORMAL.get_color(x, y);

      // Correctly decode the normal map
      let nx = (color.r as f32 / 255.0) * 2.0 - 1.0;
      let ny = (color.g as f32 / 255.0) * 2.0 - 1.0;
      let nz = color.b as f32 / 255.0; // Note: only 0 to 1 range for Z

      Vec3::new(nx, ny, nz).normalize()
    } else {
      Vec3::new(0.0, 0.0, 1.0) // Default normal if no normal map is present
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
    }
  }
}
