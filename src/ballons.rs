use color_eyre::Result;
use ratatui::{
    prelude::Color,
    widgets::canvas::{Context, Circle}
};

use rand::{thread_rng, Rng};

use crate::app::BallonPath;

#[derive(Debug, Default, Clone)]
pub struct Ballon {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: Color,
    hitpoints: f64,
    current_segment: usize,
    last_move: Vec<f64>,
    speed: f64,
    pub reward: (u16, u64), // gold, score
    pub damage: u16
}

impl Ballon {
    pub fn move_ballon(&mut self, path: &BallonPath) -> Result<bool> {
        /*
        In order for this to work the ballon must be able to move into the next segment by continuing into the previous direction. 
        This means that vertical and horizontal segments must overlap accordingly.
         */
        if self.current_segment >= path.elements.len() {
            return Ok(false);
        }
        if path.elements[self.current_segment].is_horizontal {
            let next = self.x + self.last_move[0];
            if next > path.elements[self.current_segment].x + path.elements[self.current_segment].width || next < path.elements[self.current_segment].x {
                self.current_segment += 1;
                if self.current_segment == path.elements.len() {
                    return Ok(false);
                }
                self.last_move = vec![0.0, 0.0];
                self.move_ballon(path)?;
                return Ok(true);
            }
        }
        else {
            let next = self.y + self.last_move[1];
            if next > path.elements[self.current_segment].y + path.elements[self.current_segment].height || next < path.elements[self.current_segment].y {
                self.current_segment += 1;
                if self.current_segment == path.elements.len() {
                    return Ok(false);
                }
                self.last_move = vec![0.0, 0.0];
                self.move_ballon(path)?;
                return Ok(true);
            }
        }
        
        if path.elements[self.current_segment].is_horizontal {
            if self.last_move[0] != 0.0 {
                self.x += self.last_move[0];
                return Ok(true);
            }
            if self.current_segment == 0 {
                self.x += self.speed;
                self.last_move[0] = self.speed;
                return Ok(true);
            }
            if path.elements[self.current_segment].x >= path.elements[self.current_segment - 1].x {
                self.x += self.speed;
                self.last_move[0] = self.speed;
            }
            else {
                self.x -= self.speed;
                self.last_move[0] = -self.speed;
            }
        }
        else {
            if self.last_move[1] != 0.0 {
                self.y += self.last_move[1];
                return Ok(true);
            }
            if path.elements[self.current_segment].y >= path.elements[self.current_segment - 1].y {
                self.y += self.speed;
                self.last_move[1] = self.speed;
            }
            else {
                self.y -= self.speed;
                self.last_move[1] = -self.speed;
            }
        }
        Ok(true)
    }

    pub fn reduce_hitpoints(&mut self, damge: f64) {
        self.hitpoints -= damge;
    }

    pub fn is_dead(&self) -> bool {
        if self.hitpoints <= 0.0 {
            return true;
        }
        false
    }

    pub fn generate_clone(&self) -> Self {
        Ballon {
            x: self.x,
            y: self.y,
            radius: self.radius,
            color: self.color,
            hitpoints: self.hitpoints,
            current_segment: self.current_segment,
            last_move: self.last_move.clone(),
            reward: self.reward,
            speed: self.speed,
            damage: self.damage
        }
    }

    pub fn render_self(&self, ctx: &mut Context) {
        ctx.draw(&Circle {
            x: self.x,
            y: self.y,
            radius: self.radius,
            color: self.color
        });
    }

}

#[derive(Debug, Default)]
pub struct BallonFactory {

}

impl BallonFactory {
    pub fn generate_wave(&self, round: usize, x: f64, y: f64) -> BallonWave {
        let mut rng = thread_rng();
        BallonWave {
            current: 0,
            ballons: (0..(round * 20)).map(|_index| {
                if rng.gen_range(0.0..1.0) < 0.1 {
                    self.blue_ballon(x, y)
                }
                else {
                    self.red_ballon(x, y)
                }
            }).collect(),
            ticks_since_last: 0,
            ticks_till_bloon: 30
        }
    }

    fn red_ballon(&self, x: f64, y: f64) -> Ballon {
        Ballon {
            x: x,
            y: y,
            radius: 5.0,
            color: Color::Red,
            hitpoints: 1.0,
            current_segment: 0,
            last_move: vec![0.0, 0.0],
            speed: 0.2,
            reward: (1, 1),
            damage: 1
        }
    }

    fn blue_ballon(&self, x: f64, y: f64) -> Ballon {
        Ballon {
            x: x,
            y: y,
            radius: 5.0,
            color: Color::Blue,
            hitpoints: 2.0,
            current_segment: 0,
            last_move: vec![0.0, 0.0],
            speed: 0.3,
            reward: (2, 2),
            damage: 2
        }
    }
}

#[derive (Clone, Debug)]
pub struct BallonWave {
    ballons: Vec<Ballon>,
    current: usize,
    pub ticks_since_last: u8,
    pub ticks_till_bloon: u8
}

impl Iterator for BallonWave {
    type Item = Ballon;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ticks_since_last < self.ticks_till_bloon {
            self.ticks_since_last += 1;
            return Option::from(None);
        }
        self.ticks_since_last = 0;
        if self.current == self.ballons.len() {
            Option::from(None)
        }
        else {
            let ballon = Option::from(self.ballons[self.current].clone());
            self.current += 1;
            ballon
        }
    }
}