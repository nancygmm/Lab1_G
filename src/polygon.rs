use crate::framebuffer::Framebuffer;
use crate::line::Line;
use nalgebra_glm::Vec3;

pub trait Polygon {
    fn polygon(&mut self, points: &[(usize, usize)]);
    fn fill_polygon(&mut self, points: &Vec<Vec3>);
}

impl Polygon for Framebuffer {
    fn polygon(&mut self, points: &[(usize, usize)]) {
        if points.len() < 3 {
            return;
        }

        for i in 0..points.len() {
            let (x0, y0) = points[i];
            let (x1, y1) = points[(i + 1) % points.len()];

            self.line(x0, y0, x1, y1);
        }
    }

    fn fill_polygon(&mut self, points: &Vec<Vec3>) {
        if points.is_empty() {
            return;
        }

        let min_y = points.iter().map(|p| p.y).fold(f32::INFINITY, f32::min) as usize;
        let max_y = points.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max) as usize;

        for y in min_y..=max_y {
            let mut intersections = vec![];

            for i in 0..points.len() {
                let p1 = points[i];
                let p2 = points[(i + 1) % points.len()];

                if (p1.y as usize) <= y && (p2.y as usize) > y || (p2.y as usize) <= y && (p1.y as usize) > y {
                    let x = p1.x + (y as f32 - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
                    intersections.push(x);
                }
            }

            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

            for i in (0..intersections.len()).step_by(2) {
                if i + 1 < intersections.len() {
                    let x1 = intersections[i] as usize;
                    let x2 = intersections[i + 1] as usize;

                    for x in x1..=x2 {
                        self.point(x, y);
                    }
                }
            }
        }
    }
}
