use crate::atom::Atom;
use std::collections::HashMap;
use ::rand::thread_rng;
use ::rand::Rng;
use macroquad::prelude::*;
pub struct World {
    atoms: HashMap<String, Vec<Atom>>,
    width: i32,
    height: i32,
}

impl World {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            atoms: HashMap::new(),
            width,
            height,
        }
    }

    pub fn new_group(&mut self, name: &str, amount: i32, color: Color) {
        let mut rng = thread_rng();
        let mut group: Vec<Atom> = Vec::new();
        for _i in 0..amount {
            group.push(Atom {
                x: rng.gen_range(0., self.width as f32),
                y: rng.gen_range(0., self.height as f32),
                vx: 0.0,
                vy: 0.0,
                color,
            });
        }
        self.atoms.insert(name.to_string(), group);
    }

    pub fn draw(&self) {
        for (_, j) in self.atoms.iter() {
            for a in j.iter() {
                draw_circle(a.x, a.y, 2., a.color);
            }
        }
    }

    pub fn interact(&mut self, group1: &str, group2: &str, force: f32) {

        
        let _b = self.atoms.clone();
        let mut _a = &mut self.atoms.get_mut(group1).unwrap();
        
        for i in 0.._a.len() {
            let mut fx = 0.0;
            let mut fy = 0.0;
            for j in 0.._b.clone().get(group2).unwrap().len() {
                let mut a = _a.get_mut(i).unwrap();
                let b = _b.get(group2).unwrap().get(j).unwrap();
                
                let dx = a.x - b.x;
                let dy = a.y - b.y;
                let d = f32::sqrt(dx*dx + dy*dy);
    
                if d > 0. && d < 100. {
                    let f = force * 1./d;
                    fx += f * dx;
                    fy += f * dy;
                }
                a.vx = (a.vx + fx) * 0.03;
                a.vy = (a.vy + fy) * 0.03;
                a.x += a.vx;
                a.y += a.vy;
                // if a.x <= 0. || a.x >= self.width as f32  { a.vx *= -1.; }
                // if a.y <= 0. || a.y > self.height as f32 { a.vy = -1.; }
                if a.x < 0. { a.x = self.width as f32; }
                if a.x > self.width as f32 { a.x = 0.; }
                if a.y < 0. { a.y = self.height as f32; }
                if a.y > self.height as f32 { a.y = 0.; }

            }
        }
    }
}
