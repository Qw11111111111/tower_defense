use color_eyre::owo_colors::OwoColorize;
use color_eyre::Result;
use num::ToPrimitive;
use ratatui::{
    prelude::Color,
    widgets::canvas::{Rectangle, Shape}
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

}

#[derive(Debug, Default)]
pub struct BallonFactory {

}

impl BallonFactory {
    pub fn generate_wave(&self, round: usize, x: f64, y: f64) -> Vec<Ballon> {
        let mut wave = vec![];
        for i in 0..round * 30 {
            let speed = 0.31 * round.to_f64().unwrap() - i.to_f64().unwrap() * 0.01;
            wave.push(self.red_ballon(x, y, speed));
        }
        wave
    }

    fn red_ballon(&self, x: f64, y: f64, speed: f64) -> Ballon {
        Ballon {
            x: x,
            y: y,
            radius: 5.0,
            color: Color::Red,
            hitpoints: 10.0,
            current_segment: 0,
            last_move: vec![0.0, 0.0],
            speed: speed,
            reward: (1, 1),
            damage: 1
        }
    }
}