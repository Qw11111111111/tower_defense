use num::ToPrimitive;
use ratatui::prelude::Color;

use color_eyre::Result;
use ratatui::widgets::canvas::{Circle, Context, Points, Rectangle};

use crate::app::BallonPath;
use crate::ballons::Ballon;
use crate::utils::*;

#[derive(Debug, Default, Clone)]
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
    range: f64,
    projectile_color: Color,
    projectile_size: f64,
    pub upgrades: TowerUpgradeShop
}

//find out how to do inheritance in rust (traits, ...)
impl Tower{

    pub fn dart_thrower(x: f64, y: f64) -> Self {
        Self {
            x: x,
            y: y,
            height: 5.0,
            width: 5.0, 
            color: Color::Blue,
            projectiles: vec![],
            damage_per_projectile: 10.0,
            cost: 10,
            projectile_speed: 5.0,
            ticks_per_projectile: 100,
            ticks_since_last_projectile: 0,
            range: 90.0,
            projectile_color: Color::Gray,
            projectile_size: 1.0,
            upgrades: TowerUpgradeShop::new(vec![Upgrade::RangeUpgrade(50, 20.0), Upgrade::DamageUpgrade(40, 5.0), Upgrade::FireRateUpgrade(30, 20)]),
        }
    }

    pub fn flame_thrower(x: f64, y: f64) -> Self {
        Self {
            x: x,  
            y: y,
            height: 5.0,
            width: 5.0, 
            color: Color::LightRed,
            projectiles: vec![],
            damage_per_projectile: 0.5,
            cost: 30,
            projectile_speed: 1.0,
            ticks_per_projectile: 0,
            ticks_since_last_projectile: 0,
            range: 45.0,
            projectile_color: Color::Yellow,
            projectile_size: 1.5,
            upgrades: TowerUpgradeShop::new(vec![Upgrade::RangeUpgrade(50, 20.0), Upgrade::DamageUpgrade(40, 5.0)]),
        }
    }

    pub fn collides(&self, tower: &Tower) -> bool {
        (self.x >= tower.x && self.x <= tower.x + tower.width) && (self.y >= tower.y && self.y <= tower.y + tower.height)
    }

    pub fn shoot(&mut self, ballon: &Ballon, path: &BallonPath, index: usize) -> Result<bool> {
        if self.ticks_since_last_projectile < self.ticks_per_projectile {
            self.ticks_since_last_projectile += 1;
            return Ok(true);
        }

        self.ticks_since_last_projectile = 0;

        let distance = distance_in_2d(vec![self.x, self.y + self.height / 2.0], vec![ballon.x + ballon.radius, ballon.y + ballon.radius]);

        if distance > self.range {
            return Ok(false);
        }

        let mut new_projectile = Projectile {
            x: self.x,
            y: self.y + self.height / 2.0,
            radius: self.projectile_size,
            color: self.projectile_color,
            trajectory: vec![],
            flying_time: 0,
            target_ballon: Option::from(index)
        };

        let target_set = self.get_trajectory(ballon, ballon.clone(), path, &mut new_projectile, 10)?;

        if target_set {
            self.projectiles.push(new_projectile);
        }
        Ok(true)
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

        if error <= ballon.radius * 3.0 { // i should adjust this threshhold and the max depth dynamically based on range
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

    pub fn render_self(&self, ctx: &mut Context) {
        ctx.draw(&Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            color: self.color
        });
    }

    pub fn show_upgrades(&mut self) {
        self.upgrades.show_upgrades = !self.upgrades.show_upgrades;
    }

    pub fn buy_upgrade(&mut self, y: f64, gold: &u16) -> Option<u16> {
        if let Some(upgrade) = self.upgrades.upgrade(y) {
            match upgrade {
                Upgrade::DamageUpgrade(cost, dmg) => {
                    if *gold >= cost {
                        self.damage_per_projectile += dmg;
                        return Some(cost);
                    }
                },
                Upgrade::FireRateUpgrade(cost, rate) => {
                    if *gold >= cost {
                        if self.ticks_per_projectile >= rate {
                            self.ticks_per_projectile -= rate;
                            return Some(cost);
                        }
                    }
                },
                Upgrade::RangeUpgrade(cost, range) => {
                    if *gold >= cost {
                        if self.range < 180.0 {
                            self.range += range;
                            return Some(cost);
                        }
                    }
                },
            }
        }
        None
    }

}


#[derive (Default, Debug)]
pub struct TowerShop {
    towers: Vec<Tower>,
    pub n_towers: usize
}

impl TowerShop {
    pub fn new() -> Self {
        let n = 2.0;
        Self {
            n_towers: 2,
            towers: vec![Tower::dart_thrower(180.0 / n / 2.0 - 90.0, -80.0), Tower::flame_thrower(1.0 * 180.0 / n + 180.0 / n / 2.0 - 90.0, -80.0)]
        }
    }
    
    pub fn render_self(&self, ctx: &mut Context) {
        for i in 0..self.n_towers {
            ctx.draw(&Rectangle {
                x: (i.to_f64().unwrap() * 180.0 / self.n_towers.to_f64().unwrap()) - 90.0,
                y: -90.0,
                width: 180.0 / self.n_towers.to_f64().unwrap(),
                height: 20.0,
                color: Color::White,
            });
            ctx.layer();
            self.towers[i].render_self(ctx);
        }
    }

    pub fn get_tower(&self, x: f64, gold: &u16) -> Option<Tower> {
        for i in 0..self.towers.len() {
            if x <= (i + 1).to_f64().unwrap() * 180.0 / self.n_towers.to_f64().unwrap() - 90.0 {
                if *gold >= self.towers[i].cost {
                    return Option::from(self.towers[i].clone());
                }
            }
        }
        None
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

#[derive (Debug, Default, Clone)]
pub struct TowerUpgradeShop {
    pub show_upgrades: bool,
    possible_upgrades: Vec<Upgrade>
}

impl TowerUpgradeShop {
    pub fn render_self(&self, ctx: &mut Context) {
        if !self.show_upgrades {
            return;
        }
        for (i, upgrade) in self.possible_upgrades.iter().enumerate() {
            ctx.draw(&Rectangle {
                x: 70.0,
                y: i.to_f64().unwrap() * 160.0 / self.possible_upgrades.len().to_f64().unwrap() - 70.0,
                width: 20.0,
                height: 160.0 / self.possible_upgrades.len().to_f64().unwrap(),
                color: Color::White
            });
            let x = 80.0;
            let y = i.to_f64().unwrap() * 160.0 / self.possible_upgrades.len().to_f64().unwrap() - 70.0 + 160.0 / self.possible_upgrades.len().to_f64().unwrap() / 2.0;
            upgrade.render_self(ctx, x, y);
        }
    }
    
    fn new(upgrades: Vec<Upgrade>) -> Self {
        Self {
            show_upgrades: false,
            possible_upgrades: upgrades
        }
    }

    fn upgrade(&mut self, y: f64) -> Option<Upgrade> {
        for (i, upgrade) in self.possible_upgrades.iter().enumerate() {
            if y <= (i + 1).to_f64().unwrap() * 160.0 / self.possible_upgrades.len().to_f64().unwrap() - 70.0 {
                return Some(upgrade.clone());
            }
        }
        None
    }
}

#[derive (Debug, Clone)]
pub enum Upgrade {
    RangeUpgrade(u16, f64),
    DamageUpgrade(u16, f64),
    FireRateUpgrade(u16, u8),
}

impl Upgrade {
    fn render_self(&self, ctx: &mut Context, x: f64, y: f64) {
        match self {
            Upgrade::DamageUpgrade(_, _) => {
                ctx.draw(&Circle {
                    x: x - 1.0, 
                    y: y - 1.0,
                    radius: 1.0,
                    color: Color::Red
                })
            },
            Upgrade::FireRateUpgrade(_, _) => {
                ctx.draw(&Points {
                    coords: &[(x, y), (x - 2.0, y), (x + 2.0, y)],
                    color: Color::Red
                })
            },
            Upgrade::RangeUpgrade(_, _) => {
                ctx.draw(&Circle {
                    x: x - 3.0,
                    y: y - 3.0,
                    radius: 3.0,
                    color: Color::White
                })
            },
        }
    }
}