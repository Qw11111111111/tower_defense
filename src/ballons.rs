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
    current_segment: usize,
    last_move: Vec<f64>
}

impl Ballon {
    pub fn move_ballon(&mut self, path: &BallonPath) -> bool {
        if path.elements[self.current_segment].is_horizontal {
            let next = self.x + self.last_move[0];
            if next > path.elements[self.current_segment].x + path.elements[self.current_segment].width || next < path.elements[self.current_segment].x {
                self.current_segment += 1;
                if self.current_segment == path.elements.len() {
                    return false;
                }
                self.last_move = vec![0.0, 0.0];
                self.move_ballon(path);
                return true;
            }
        }
        else {
            let next = self.y + self.last_move[1];
            if next > path.elements[self.current_segment].y + path.elements[self.current_segment].height || next < path.elements[self.current_segment].y {
                self.current_segment += 1;
                if self.current_segment == path.elements.len() {
                    return false;
                }
                self.last_move = vec![0.0, 0.0];
                self.move_ballon(path);
                return true;
            }
        }
        
        if path.elements[self.current_segment].is_horizontal {
            if self.last_move[0] != 0.0 {
                self.x += self.last_move[0];
                return true;
            }
            if self.current_segment == 0 {
                self.x += 1.0;
                self.last_move[0] = 1.0;
                return true;
            }
            if path.elements[self.current_segment].x >= path.elements[self.current_segment - 1].x {
                self.x += 1.0;
                self.last_move[0] = 1.0;
            }
            else {
                self.x -= 1.0;
                self.last_move[0] = -1.0;
            }
        }
        else {
            if self.last_move[1] != 0.0 {
                self.y += self.last_move[1];
                return true;
            }
            if path.elements[self.current_segment].y >= path.elements[self.current_segment - 1].y {
                self.y += 1.0;
                self.last_move[1] = 1.0;
            }
            else {
                self.y -= 1.0;
                self.last_move[1] = -1.0;
            }
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
            current_segment: 0,
            last_move: vec![0.0, 0.0]
        }
    }
}