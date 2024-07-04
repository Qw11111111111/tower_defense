use ratatui::{
    prelude::Color,
    widgets::canvas::{Rectangle, Shape}
};

use crate::app::BallonPath;

#[derive(Debug, Default, Clone)]
pub struct Ballon {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: Color,
    hitpoints: u64,
    current_segment: usize
}

impl Ballon {
    pub fn move_ballon(&mut self, path: &BallonPath) -> bool {
        let next = self.x + 1.0;
        if next > path.elements[self.current_segment].x + path.elements[self.current_segment].width {
            let next_segment = self.current_segment + 1;
            if next_segment >= path.elements.len() {
                return false;
            }
            if path.elements[next_segment].y > path.elements[self.current_segment].y {
                self.y += 1.0;
            }
            else if path.elements[next_segment].y < path.elements[self.current_segment].y {
                self.y -= 1.0;
            }
            else if path.elements[next_segment].x > path.elements[self.current_segment].x {
                self.x += 1.0;
            }
            self.current_segment += 1;
        }
        else {
            self.x += 1.0;
        }
        true
    }
}

#[derive(Debug, Default)]
pub struct BallonFactory {

}

impl BallonFactory {
    pub fn generate_wave(&self, round: usize, x: f64, y: f64) -> Vec<Ballon> {
        vec![self.red_ballon(x, y); round * 10]
    }

    fn red_ballon(&self, x: f64, y: f64) -> Ballon {
        Ballon {
            x: x,
            y: y,
            radius: 10.0,
            color: Color::Red,
            hitpoints: 5,
            current_segment: 0
        }
    }
}