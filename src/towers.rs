use num::ToPrimitive;
use ratatui::prelude::Color;

use crate::ballons::Ballon;

#[derive(Debug, Default)]
pub struct Tower {
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub width: f64,
    pub color: Color,
    pub projectiles: Vec<Projectile>,
    damage_per_tick: f64,
    pub cost: u16 
}

//find out how to do inheritance in rust (traits, ...)
impl Tower {
    pub fn new(x: f64, y: f64) -> Self {
        Tower {
            x: x,
            y: y,
            height: 5.0,
            width: 5.0, 
            color: Color::Blue,
            projectiles: vec![Projectile::default(); 2],
            damage_per_tick: 1.0,
            cost: 10
        }
    }

    pub fn collides(&self, tower: &Tower) -> bool {
        (self.x >= tower.x && self.x <= tower.x + tower.width) && (self.y >= tower.y && self.y <= tower.y + tower.height)
    }

    pub fn shoot(&mut self, ballon: &mut Ballon) {
        self.projectiles.push(Projectile {
            x: ballon.x,
            y: ballon.y,
            radius: 2.0,
            color: Color::Magenta
        });
        self.projectiles.push(Projectile {
            x: self.x,
            y: self.y + self.height / 2.0,
            radius: 1.0,
            color: Color::Red
        });
        ballon.reduce_hitpoints(self.damage_per_tick);
        self.projectiles.remove(0);
        self.projectiles.remove(0);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Projectile {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: Color
}