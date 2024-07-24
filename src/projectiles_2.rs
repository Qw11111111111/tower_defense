use ratatui::style::Color;


pub trait Projectile {
    fn move_(&mut self);
}

#[derive(Debug, Clone)]
pub enum Projectiles {
    SimpleProjectile,
}



#[derive(Debug, Default, Clone)]
pub struct SimpleProjectile {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: Color,
    pub trajectory: Vec<f64>,
    pub flying_time: i32,
    pub target_ballon: Option<usize>
}

impl Projectile for SimpleProjectile {
    fn move_(&mut self) {
        self.x += self.trajectory[0];
        self.y += self.trajectory[1];
        self.flying_time -= 1;
    }
}

#[derive(Debug, Default, Clone)]
pub struct OtherProjectile {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: Color,
    pub trajectory: Vec<f64>,
    pub flying_time: i32,
    pub target_ballon: Option<usize>
}

impl Projectile for OtherProjectile {
    fn move_(&mut self) {
        self.x += self.trajectory[0];
        self.y += self.trajectory[1];
        self.flying_time -= 1;
    }
}

fn test() {
    let projectiles: &dyn Projectile;
    //projectiles = SimpleProjectile{};
    //projectiles.move_();
}



struct TestStruct {
    projectiles: Vec<&'static dyn Projectile>,
}

impl TestStruct {
    fn new(projectiles: Vec<&'static dyn Projectile>) -> Self {
        TestStruct {
            projectiles: projectiles,
        }
    }

    fn move_pro(&mut self) {
        for p in self.projectiles.iter_mut() {
            //p.move_();
        }
    }
}
