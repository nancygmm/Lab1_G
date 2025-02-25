extern crate nalgebra_glm as glm;

use glm::Vec3;

mod framebuffer;
mod line;
mod bmp;
mod polygon;

use crate::polygon::Polygon;
use crate::framebuffer::Framebuffer;
use crate::line::Line;
use crate::bmp::WriteBmp;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    let poligono1 = vec![
        (165, 380), 
        (185, 360), 
        (180, 330), 
        (207, 345), 
        (233, 330),
        (230, 360), 
        (250, 380), 
        (220, 385), 
        (205, 410), 
        (193, 383)
    ];

    let poligono2 = vec![
        (321, 335),
        (288, 286),
        (339, 251),
        (374, 302)
    ];

    let poligono3 = vec![
        (377, 249),
        (411, 197),
        (436, 249)
    ];

    let poligono4 = vec![
        (413, 177),
        (448, 159),
        (502, 88),
        (553, 53),
        (535, 36),
        (676, 37),
        (660, 52),
        (750, 145),
        (761, 179),
        (672, 192),
        (659, 214),
        (615, 214),
        (632, 230),
        (580, 230),
        (597, 215),
        (552, 214),
        (517, 144),
        (466, 180)
    ];

    let poligono5 = vec![
        (682, 175),
        (708, 120),
        (735, 148),
        (739, 170)
    ];

    let poligono1_vec3: Vec<Vec3> = poligono1.iter()
        .map(|&(x, y)| Vec3::new(x as f32, y as f32, 0.0))
        .collect();

    let poligono2_vec3: Vec<Vec3> = poligono2.iter()
        .map(|&(x, y)| Vec3::new(x as f32, y as f32, 0.0))
        .collect();
    
    let poligono3_vec3: Vec<Vec3> = poligono3.iter()
        .map(|&(x, y)| Vec3::new(x as f32, y as f32, 0.0))
        .collect();

    let poligono4_vec3: Vec<Vec3> = poligono4.iter()
        .map(|&(x, y)| Vec3::new(x as f32, y as f32, 0.0))
        .collect();

    let poligono5_vec3: Vec<Vec3> = poligono5.iter()
        .map(|&(x, y)| Vec3::new(x as f32, y as f32, 0.0))
        .collect();

    framebuffer.set_current_color(0x00FFFF);
    framebuffer.fill_polygon(&poligono1_vec3);

    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&poligono1);

    framebuffer.set_current_color(0xFF0000);
    framebuffer.fill_polygon(&poligono2_vec3);

    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&poligono2);

    framebuffer.set_current_color(0x0000FF);
    framebuffer.fill_polygon(&poligono3_vec3);

    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&poligono3);

    framebuffer.set_current_color(0x00FF00);
    framebuffer.fill_polygon(&poligono4_vec3);

    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&poligono4);

    framebuffer.set_current_color(0x000000);
    framebuffer.fill_polygon(&poligono5_vec3);

    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&poligono5);

    let _ = framebuffer.render_buffer("output.bmp");
}
