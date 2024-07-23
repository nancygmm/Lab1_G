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

    framebuffer.set_current_color(0xFFFFFF);

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

    framebuffer.polygon(&poligono1);

    let _ = framebuffer.render_buffer("output.bmp");
}   