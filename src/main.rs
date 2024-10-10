use crate::texture::Texture;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{normalize, Vec3};
use std::f32::consts::PI;
use std::time::Duration;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

mod camera;
mod color;
mod cube;
mod framebuffer;
mod light;
mod material;
mod ray_intersect;
mod sphere;
mod texture;

use crate::cube::Cube;
use camera::Camera;
use color::Color;
use framebuffer::Framebuffer;
use light::Light;
use material::Material;
use ray_intersect::{Intersect, RayIntersect};

const ORIGIN_BIAS: f32 = 1e-4;

fn offset_origin(intersect: &Intersect, direction: &Vec3) -> Vec3 {
    let offset = intersect.normal * ORIGIN_BIAS;
    if direction.dot(&intersect.normal) < 0.0 {
        intersect.point - offset
    } else {
        intersect.point + offset
    }
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);

    let (n_cosi, eta, n_normal);

    if cosi < 0.0 {
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    } else {
        n_cosi = cosi;
        eta = eta_t; 
        n_normal = *normal;
    }

    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);

    if k < 0.0 {
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[&dyn RayIntersect], 
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let light_distance = (light.position - intersect.point).magnitude();

    let shadow_ray_origin = offset_origin(intersect, &light_dir);
    let mut shadow_intensity = 0.0;

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting && shadow_intersect.distance < light_distance {
            let distance_ratio = shadow_intersect.distance / light_distance;
            shadow_intensity = 1.0 - distance_ratio.powf(2.0).min(1.0);
            break;
        }
    }

    shadow_intensity * 0.9
}


pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[&dyn RayIntersect],
    lights: &[Light],
    depth: u32,
    skybox_texture: &Texture,
) -> Color {
    if depth > 3 {
        let dir = ray_direction.normalize();
        let theta = dir.z.atan2(dir.x);
        let phi = dir.y.asin();
        let u = (theta + PI) / (2.0 * PI);
        let v = (phi + PI / 2.0) / PI;
        return skybox_texture.get_color(u, v);
    }

    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let i = object.ray_intersect(ray_origin, ray_direction);
        if i.is_intersecting && i.distance < zbuffer {
            zbuffer = i.distance;
            intersect = i;
        }
    }

    if !intersect.is_intersecting {
        let dir = ray_direction.normalize();
        let theta = dir.z.atan2(dir.x);
        let phi = dir.y.asin();
        let u = (theta + PI) / (2.0 * PI);
        let v = (phi + PI / 2.0) / PI;
        return skybox_texture.get_color(u, v);
    }

    let view_dir = (ray_origin - intersect.point).normalize();

    let mut final_color = Color::black();

    for light in lights {
        let light_dir = (light.position - intersect.point).normalize();
        let reflect_dir = reflect(&-light_dir, &intersect.normal).normalize();
    
        let shadow_intensity = cast_shadow(&intersect, light, objects);
        let light_intensity = light.intensity * (1.0 - shadow_intensity);
    
        let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);
        let diffuse_color = intersect
            .material
            .get_diffuse_color(intersect.u, intersect.v);
        let diffuse =
            diffuse_color * intersect.material.albedo[0] * diffuse_intensity * light_intensity;
    
        let specular_intensity = view_dir
            .dot(&reflect_dir)
            .max(0.0)
            .powf(intersect.material.specular);
        let specular =
            light.color * intersect.material.albedo[1] * specular_intensity * light_intensity;
    
        final_color += diffuse + specular;
    }

    let mut reflect_color = Color::black();
    let reflectivity = intersect.material.albedo[2];
    if reflectivity > 0.0 {
        let reflect_dir = reflect(&ray_direction, &intersect.normal).normalize();
        let reflect_origin = offset_origin(&intersect, &reflect_dir);
        reflect_color =
            cast_ray(&reflect_origin, &reflect_dir, objects, lights, depth + 1, skybox_texture);
    }

    let mut refract_color = Color::black();
    let transparency = intersect.material.albedo[3];
    if transparency > 0.0 {
        let refract_dir = refract(
            &ray_direction,
            &intersect.normal,
            intersect.material.refractive_index,
        );
        let refract_origin = offset_origin(&intersect, &refract_dir);
        refract_color =
            cast_ray(&refract_origin, &refract_dir, objects, lights, depth + 1, skybox_texture);
    }

    final_color = final_color * (1.0 - reflectivity - transparency)
        + (reflect_color * reflectivity)
        + (refract_color * transparency);

    final_color
}


pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[&dyn RayIntersect],
    camera: &Camera,
    lights: &[Light],
    skybox_texture: &Texture,
) {
    let width = framebuffer.width as usize;
    let height = framebuffer.height as usize;
    let aspect_ratio = width as f32 / height as f32;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan();

    let buffer = Arc::new(Mutex::new(&mut framebuffer.buffer));

    let pixels: Vec<usize> = (0..(width * height)).collect();

    pixels.par_iter().for_each(|&i| {
        let x = i % width;
        let y = i / width;

        let screen_x = (2.0 * x as f32) / width as f32 - 1.0;
        let screen_y = -(2.0 * y as f32) / height as f32 + 1.0;

        let screen_x = screen_x * aspect_ratio * perspective_scale;
        let screen_y = screen_y * perspective_scale;

        let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));
        let rotated_direction = camera.basis_change(&ray_direction);

        let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, lights, 0, skybox_texture);

        let mut buffer = buffer.lock().unwrap();
        buffer[i] = pixel_color.to_hex();
    });
}



fn main() {
    
    let snow_texture = Arc::new(Texture::new("assets/snow.png"));
    let snow_material =
        Material::new_with_texture(2.0, [0.9, 0.1, 0.0, 0.0], 0.0, snow_texture, None);

    let ice_texture = Arc::new(Texture::new("assets/ice.png"));
    let ice_material =
        Material::new_with_texture(2.0, [0.3, 0.3, 0.0, 0.4], 0.5, ice_texture, None);

    
    let skybox_texture = Arc::new(Texture::new("assets/snowy.jpg"));
    
    let mut objects: Vec<Box<dyn RayIntersect>> = Vec::new();
    let rows = 9;
    let cols = 9;
    let size = 2.0; 
    let x_offset = -(cols as f32) * size / 2.0; 
    let z_offset = -(rows as f32) * size / 2.0; 

    
    for row in 0..rows {
        for col in 0..cols {
            let x = x_offset + col as f32 * size;
            let z = z_offset + row as f32 * size;
            let cube = Cube {
                min: Vec3::new(x, -size / 2.0, z),
                max: Vec3::new(x + size, size / 2.0, z + size),
                material: snow_material.clone(),
            };
            objects.push(Box::new(cube));
        }
    }

    
    let pattern_positions_level_1_and_2 = vec![
        
        (2, 2), (2, 3), (2, 4), (2, 5), (2, 6),
        
        (3, 1), (3, 7),
        
        (4, 1), (4, 7),
        
        (5, 1), (5, 7),
        
        (6, 2), (6, 3), (6, 5), (6, 6),
    ];

    let pattern_positions_level_3 = vec![
        
        (2, 3), (2, 4), (2, 5),
        
        (3, 2), (3, 3), (3, 4), (3, 5), (3, 6),
        
        (4, 1), (4, 2), (4, 3), (4, 4), (4, 5), (4, 6), (4, 7),
        
        (5, 2), (5, 3), (5, 4), (5, 5), (5, 6),
        
        (6, 3), (6, 4), (6, 5),
    ];

    let pattern_positions_level_4 = vec![
        
        (3, 4),
        
        (4, 3), (4, 4), (4, 5),
        
        (5, 4),
    ];

    
    struct LevelPattern {
        y_level: f32,
        positions: Vec<(usize, usize)>,
    }

    let levels = vec![
        LevelPattern {
            y_level: size / 2.0,
            positions: pattern_positions_level_1_and_2.clone(),
        },
        LevelPattern {
            y_level: size * 1.5,
            positions: pattern_positions_level_1_and_2.clone(),
        },
        LevelPattern {
            y_level: size * 2.5,
            positions: pattern_positions_level_3.clone(),
        },
        LevelPattern {
            y_level: size * 3.4,
            positions: pattern_positions_level_4.clone(),
        },
    ];

    
    for level in levels {
        for (row, col) in &level.positions {
            let x = x_offset + *col as f32 * size;
            let z = z_offset + *row as f32 * size;
            let cube = Cube {
                min: Vec3::new(x, level.y_level, z),
                max: Vec3::new(x + size, level.y_level + size, z + size),
                material: ice_material.clone(),
            };
            objects.push(Box::new(cube));
        }
    }

    
    let mut camera = Camera::new(
        Vec3::new(0.0, 15.0, 30.0), 
        Vec3::new(0.0, 0.0, 0.0),   
        Vec3::new(0.0, 1.0, 0.0),   
    );
    let rotation_speed = PI / 50.0;
    let zoom_speed = 0.1;

    let light1 = Light::new(Vec3::new(20.0, 30.0, 20.0), Color::new(177, 182, 250), 15.0);
    let light2 = Light::new(Vec3::new(-20.0, 30.0, -20.0), Color::new(255, 180, 180), 10.0);
    let lights = vec![light1, light2];

    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Snonwy Night Scene - Press ESC to exit",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        
        let object_refs: Vec<&dyn RayIntersect> = objects.iter().map(|obj| obj.as_ref()).collect();

        
        if window.is_key_down(Key::Left) {
            camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Right) {
            camera.orbit(-rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, -rotation_speed);
        }
        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, rotation_speed);
        }

        if window.is_key_down(Key::Q) {
            camera.zoom(zoom_speed);
        }
        if window.is_key_down(Key::E) {
            camera.zoom(-zoom_speed);
        }

        if camera.is_changed() {
            render(
                &mut framebuffer,
                &object_refs,
                &camera,
                &lights,  
                &skybox_texture,
            );
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}