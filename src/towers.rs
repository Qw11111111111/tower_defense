use num::ToPrimitive;
use ratatui::prelude::Color;

use color_eyre::{
    eyre::WrapErr, Result
};

use std::rc::Rc;

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
    damage_per_projectile: f64,
    pub cost: u16,
    projectile_speed: f64,
    ticks_per_projectile: u8,
    ticks_since_last_projectile: u8,
    range: Option<f64>
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
            damage_per_projectile: 10.0,
            cost: 10,
            projectile_speed: 5.0,
            ticks_per_projectile: 3 ,
            ticks_since_last_projectile: 0,
            range: Option::from(None)
        }
    }

    pub fn collides(&self, tower: &Tower) -> bool {
        (self.x >= tower.x && self.x <= tower.x + tower.width) && (self.y >= tower.y && self.y <= tower.y + tower.height)
    }

    pub fn shoot(&mut self, ballon: &Ballon, path: &BallonPath, index: usize) -> Result<()> {
        if self.ticks_since_last_projectile < self.ticks_per_projectile {
            self.ticks_since_last_projectile += 1;
            return Ok(());
        }

        self.ticks_since_last_projectile = 0;

        let distance = distance_in_2d(vec![self.x, self.y + self.height / 2.0], vec![ballon.x + ballon.radius, ballon.y + ballon.radius]);

        match self.range {
            None => {},
            Some(value) => {
                if distance > value {
                    return Ok(());
                }
            }
        }

        let mut new_projectile = Projectile {
            x: self.x,
            y: self.y + self.height / 2.0,
            radius: 1.0,
            color: Color::Magenta,
            trajectory: vec![],
            flying_time: 0,
            target_ballon: Option::from(index)
        };

        let target_set = self.get_trajectory(ballon, ballon.clone(), path, &mut new_projectile, 20)?;

        if target_set {
            self.projectiles.push(new_projectile);
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
            if self.projectiles[i - k].flying_time < 0 {
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

        let current_distance = distance_in_2d(vec![self.x, self.y + self.height / 2.0], vec![current_target.x + ballon.radius, current_target.y + ballon.radius]);

        let flying_time = current_distance / self.projectile_speed;

        let mut ballon_at_hit_time = ballon.clone();

        for _ in 0..flying_time.round().to_i32().unwrap() {
            ballon_at_hit_time.move_ballon(path)?;
        }

        let error = distance_in_2d(vec![ballon.x + ballon.radius, ballon.y + ballon.radius], vec![ballon_at_hit_time.x + ballon.radius, ballon_at_hit_time.y + ballon.radius]);

        if error <= ballon.radius * 3.0 {
            projectile.flying_time = flying_time.round().to_i32().unwrap();
            projectile.trajectory = vec![((ballon.x + ballon.radius) - self.x) / flying_time.round(), ((ballon.y + ballon.radius) - self.y + self.height) / flying_time.round()];
            return Ok(true);
        }

        self.get_trajectory(ballon, ballon_at_hit_time, path, projectile, depth - 1)?;

        Ok(false)
    }

    pub fn calculate_damage(&self, index: usize) -> f64 {
        let mut dmg = 0.0;
        for projectile in self.projectiles.iter() {
            match projectile.target_ballon {
                None => continue,
                Some(value) => {
                    if projectile.flying_time == 0 && value == index {
                        dmg += self.damage_per_projectile;
                    }
                }
            }
        }
        dmg
    }

    pub fn remove_target_of_projectile(&mut self, index: usize) -> Result<()> {
        for projectile in self.projectiles.iter_mut() {
            match projectile.target_ballon {
                None => continue,
                Some(value) => {
                    if value == index {
                        projectile.target_ballon = Option::from(None);
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Projectile {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: Color,
    trajectory: Vec<f64>,
    pub flying_time: i32,
    target_ballon: Option<usize>
}

impl Projectile {
    fn move_(&mut self) {
        self.x += self.trajectory[0];
        self.y += self.trajectory[1];
        self.flying_time -= 1;
    }
}