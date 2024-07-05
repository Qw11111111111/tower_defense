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
    damage_per_tick: f64
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
            projectiles: vec![],
            damage_per_tick: 1.0
        }
    }

    pub fn shoot(&self, ballon: &mut Ballon) {
        ballon.reduce_hitpoints(self.damage_per_tick);
    }
}

#[derive(Debug, Default)]
pub struct Projectile {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: Color
}