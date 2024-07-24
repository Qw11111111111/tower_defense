use ratatui::{style::Color, widgets::canvas::{Circle, Context}};


#[derive (Debug)]
pub enum Projectiles {
    ParentProjectile {

    }
}

impl Projectiles {
    fn move_(&mut self) {
        self::ParentProjectile
    }
}

struct ParentProjectile {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: Color,
    pub trajectory: Vec<f64>,
    pub flying_time: i32,
    pub target_ballon: Option<usize>
}


pub trait Projectile {
    fn move_(&mut self);
    fn render(&mut self, ctx: &mut Context);
    fn new(x: f64, y: f64) -> Self;
    fn set_trajectory(&mut self, trajectory: Vec<f64>);
    fn set_target_ballon(&mut self, target: Option<usize>);
    fn set_flying_time(&mut self, time: i32);
}

impl Projectile for ParentProjectile {
    fn move_(&mut self) {
        self.x += self.trajectory[0];
        self.y += self.trajectory[1];
        self.flying_time -= 1;
    }
    
    fn render(&mut self, ctx: &mut Context) {
        panic!("not implemented");
    }

    fn set_target_ballon(&mut self, target: Option<usize>) {
        self.target_ballon = target;
    }

    fn set_trajectory(&mut self, trajectory: Vec<f64>) {
        self.trajectory = trajectory;
    }

    fn set_flying_time(&mut self, time: i32) {
        self.flying_time = time;
    }

    fn new(x: f64, y: f64) -> Self {
        ParentProjectile {
            x: x,
            y: y,
            radius: 0.0,
            color: Color::Black,
            trajectory: vec![],
            flying_time: 0,
            target_ballon: Option::from(None)
        }
    }
}

pub struct Bullet {
    projectile: ParentProjectile,
    radius: f64, 
    color: Color,
}

impl Projectile for Bullet {
    fn move_(&mut self) {
        self.projectile.move_();
    }

    fn new(x: f64, y: f64) -> Self {
        Bullet {
            projectile: ParentProjectile::new(x, y),
            radius: 5.0,
            color: Color::White
        }
    }

    fn render(&mut self, ctx: &mut Context) {
        ctx.draw(&Circle {
            x: self.projectile.x,
            y: self.projectile.y,
            radius: self.radius,
            color: self.color
        });
    }

    fn set_target_ballon(&mut self, target: Option<usize>) {
        self.projectile.set_target_ballon(target);
    }

    fn set_trajectory(&mut self, trajectory: Vec<f64>) {
        self.projectile.set_trajectory(trajectory);
    }

    fn set_flying_time(&mut self, time: i32) {
        self.projectile.set_flying_time(time);
    }
}

pub struct Ray {
    projectile: ParentProjectile,
    color: Color,
}

impl Projectile for Ray {
    fn move_(&mut self) {
        
    }

    fn new(x: f64, y: f64) -> Self {
        Ray {
            projectile: ParentProjectile::new(x, y),
            color: Color::White
        }
    }

    fn render(&mut self, ctx: &mut Context) {
    }

    fn set_target_ballon(&mut self, target: Option<usize>) {
        self.projectile.set_target_ballon(target);
    }

    fn set_trajectory(&mut self, trajectory: Vec<f64>) {
        self.projectile.set_trajectory(trajectory);
    }

    fn set_flying_time(&mut self, time: i32) {
        self.projectile.set_flying_time(time);
    }
}

struct Test {
    projectiles: Vec<Box<Projectiles>>
}

impl Test {
    fn huh(&self) {
        for p in self.projectiles.iter() {
            p
        }
    }
}