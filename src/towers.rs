use num::ToPrimitive;
use ratatui::prelude::Color;

use color_eyre::{
    eyre::WrapErr, Result
};

use crate::app::BallonPath;
use crate::ballons::Ballon;
use crate::utils::*;

#[derive(Debug, Default)]
pub struct Tower {
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub width: f64,
    pub color: Color,
    pub projectiles: Vec<Projectile>,
    damage_per_tick: f64,
    pub cost: u16,
    projectile_speed: f64 
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
            cost: 10,
            projectile_speed: 1.0

        }
    }

    pub fn collides(&self, tower: &Tower) -> bool {
        (self.x >= tower.x && self.x <= tower.x + tower.width) && (self.y >= tower.y && self.y <= tower.y + tower.height)
    }

    pub fn shoot(&mut self, ballon: &mut Ballon, path: &BallonPath) -> Result<()> {

        let mut new_projectile = Projectile {
            x: self.x,
            y: self.y + self.height / 2.0,
            radius: 1.0,
            color: Color::Magenta,
            trajectory: vec![],
            flying_time: 0
        };

        let target_set = self.get_trajectory(ballon, ballon.clone(), path, &mut new_projectile, 10)?;

        if target_set {
            self.projectiles.push(new_projectile);
        }

        for projectile in self.projectiles.iter() {
            if projectile.flying_time == 0 {
                ballon.reduce_hitpoints(self.damage_per_tick);
            }
        }

        /*
        self.projectiles.push(Projectile {
            x: ballon.x,
            y: ballon.y,
            radius: 2.0,
            color: Color::Magenta,
            target: vec![]
        });
        self.projectiles.push(Projectile {
            x: self.x,
            y: self.y + self.height / 2.0,
            radius: 1.0,
            color: Color::Red,
            target: vec![]
        });
        ballon.reduce_hitpoints(self.damage_per_tick);
        self.projectiles.remove(0);
        self.projectiles.remove(0);
        */

        Ok(())
    }

    pub fn handle_projectile(&mut self) -> Result<()> {
        let mut k = 0;
        for i in 0..self.projectiles.len() {
            if self.projectiles[i - k].flying_time <= 0 {
                self.projectiles.remove(i - k);
                k += 1;
                continue;
            }
            self.projectiles[i - k].move_();
        };
        Ok(())
    }

    fn get_trajectory(&self, ballon: &Ballon, current_target: Ballon, path: &BallonPath, projectile: &mut Projectile, depth: u8) -> Result<bool> {

        if depth == 0 {
            return Ok(false);
        }

        let current_distance = distance_in_2d(vec![self.x, self.y + self.height / 2.0], vec![current_target.x + ballon.radius / 2.0, current_target.y + ballon.radius / 2.0]);

        let flying_time = current_distance / self.projectile_speed;

        let mut ballon_at_hit_time = ballon.clone();

        for _ in 0..flying_time.round().to_i32().unwrap() {
            ballon_at_hit_time.move_ballon(path)?;
        }

        let error = distance_in_2d(vec![ballon.x + ballon.radius / 2.0, ballon.y + ballon.radius / 2.0], vec![ballon_at_hit_time.x + ballon.radius / 2.0, ballon_at_hit_time.y + ballon.radius / 2.0]);

        if error <= ballon.radius * 2.0 {
            projectile.flying_time = flying_time.round().to_i32().unwrap();
            projectile.trajectory = vec![((ballon.x + ballon.radius / 2.0) - self.x) / flying_time.round(), ((ballon.y + ballon.radius / 2.0) - self.y + self.height / 2.0) / flying_time.round()];
            return Ok(true);
        }

        self.get_trajectory(ballon, ballon_at_hit_time, path, projectile, depth - 1)?;

        Ok(false)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Projectile {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: Color,
    trajectory: Vec<f64>,
    flying_time: i32
}

impl Projectile {
    fn move_(&mut self) {
        self.x += self.trajectory[0];
        self.y += self.trajectory[1];
        self.flying_time -= 1;
    }
}