use ratatui::{
    prelude::Color,
    widgets::canvas::{Rectangle, Shape}
};

#[derive(Debug, Default)]
pub struct Ballon {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: Color,
}

#[derive(Debug, Default)]
pub struct BallonFactory {

}

impl BallonFactory {
    fn generate_wave() -> Vec<Ballon> {
        vec![]
    }
}