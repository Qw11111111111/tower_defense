use ratatui::prelude::Color;


#[derive(Debug, Default)]
pub struct Tower {
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub width: f64,
    pub color: Color,
    pub projectiles: Vec<Projectile>
}

//find out how to do inheritance in rust (traits, ...)
impl Tower {
    
}

#[derive(Debug, Default)]
pub struct Projectile {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: Color
}