use {
    crate::{
        balloons::*, towers::*, tui
    }, 
    color_eyre::{
        eyre::WrapErr, Result
    }, 
    crossterm::{
        event::{
            self,
            Event,
            KeyCode,
            KeyEvent,
            KeyEventKind,
            MouseButton,
            MouseEvent,
            MouseEventKind
        },
        terminal::size
    }, 
    ratatui::{
        prelude::{
            text, 
            Alignment, 
            Buffer, 
            Frame, 
            Rect, 
            Stylize, 
            Widget
        }, 
            style::Color, 
            widgets::{
                block::{Block, Position, Title}, 
                canvas::{self, Canvas, Circle, Rectangle}, 
                Borders, 
                Paragraph
            }
    }, 
    std::time::Duration
};

#[derive(Debug, Default)]
pub struct App {
    pub score: u64,
    pub highscore: u64,
    exit: bool,
    on_pause: bool,
    dead: bool,
    path: BalloonPath,
    balloons: Vec<Balloon>,
    towers: Vec<Tower>,
    balloon_factory: BalloonFactory,
    round: usize,
    max_cols: u16,
    max_rows: u16,
    gold: u16,
    hitpoints: u16,
    new_tower: Option<Tower>,
    tower_shop: TowerShop,
    upgrade_shop_open: Option<usize>,
    tower_shop_open: bool,
    restart: bool
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
        where
            Self: Sized {

                let instructions = Title::from(text::Line::from(vec![
                    " exit:".bold(),
                    " <q> ".bold(),
                    " restart:".bold(),
                    " <Enter> ".bold()
                ]));

                let block = Block::default()
                    .borders(Borders::NONE)
                    .title(Title::from(" tower defense ".bold())
                        .alignment(Alignment::Center)
                        .position(Position::Top))
                    .title(instructions
                        .alignment(Alignment::Center)
                        .position(Position::Bottom))
                    .bg(Color::Black);

                Paragraph::new(text::Line::from(vec!["score: ".bold(), self.score.to_string().into(), " | Gold: ".bold(), self.gold.to_string().into(), " | wave: ".bold(), self.round.to_string().into(), " | hitpoints: ".bold(), self.hitpoints.to_string().into()]))
                    .alignment(Alignment::Left)
                    .block(block.clone())
                    .render(area, buf);

                Paragraph::new(text::Line::from(vec!["highscore: ".bold(), self.highscore.to_string().into()]))
                    .alignment(Alignment::Right)
                    .block(block.clone())
                    .render(area, buf);
            

                if !self.dead {
                    Canvas::default()
                        .block(block.clone())
                        .x_bounds([-90.0, 90.0])
                        .y_bounds([-90.0, 90.0])
                        .background_color(Color::Black)
                        .paint(|ctx| {
                            for rect in self.path.elements.iter() { // draw the path of the balloons
                                ctx.draw(&Rectangle {
                                    x: rect.x,
                                    y: rect.y,
                                    height: rect.height,
                                    width: rect.width,
                                    color: Color::White
                                })
                            }
                            ctx.layer();
                            for ballon in self.balloons.iter() { // draw the balloons
                                ballon.render_self(ctx);
                            }
                            ctx.layer();
                            for tower in self.towers.iter() { // draw all projectiles
                                for projectile in tower.projectiles.iter() {
                                    if projectile.flying_time == 0 {
                                        continue;
                                    }
                                    ctx.draw(&Circle {
                                        x: projectile.x,
                                        y: projectile.y,
                                        radius: projectile.radius,
                                        color: projectile.color
                                    })
                                }
                            }
                            ctx.layer();
                            for tower in self.towers.iter() { // draw the towers
                                tower.render_self(ctx);
                            }
                            ctx.layer();
                            if self.tower_shop_open {
                                self.tower_shop.render_self(ctx);
                                ctx.draw(&Rectangle {
                                    x: -90.0,
                                    y: -70.0,
                                    width: 5.0,
                                    height: 5.0,
                                    color: Color::White
                                });
                                ctx.draw(&canvas::Line {
                                    x1: -89.0,
                                    y1: -66.0,
                                    x2: -87.5,
                                    y2: -69.0,
                                    color: Color::White
                                });
                                ctx.draw(&canvas::Line {
                                    x1: -86.0,
                                    y1: -66.0,
                                    x2: -87.5,
                                    y2: -69.0,
                                    color: Color::White
                                })
                            }
                            else {
                                ctx.draw(&Rectangle {
                                    x: -90.0,
                                    y: -90.0,
                                    width: 5.0,
                                    height: 5.0,
                                    color: Color::White
                                });
                                ctx.draw(&canvas::Line {
                                    x1: -89.0,
                                    y1: -89.0,
                                    x2: -87.5,
                                    y2: -86.0,
                                    color: Color::White
                                });
                                ctx.draw(&canvas::Line {
                                    x1: -86.0,
                                    y1: -89.0,
                                    x2: -87.5,
                                    y2: -86.0,
                                    color: Color::White
                                })
                            }
                            ctx.layer();
                            for tower in self.towers.iter() {
                                if self.tower_shop_open {
                                    tower.upgrades.render_self(ctx, -20.0);
                                }
                                else {
                                    tower.upgrades.render_self(ctx, 0.0);
                                }
                            }
                            ctx.layer();
                            match &self.new_tower {
                                None => (),
                                Some(tower) => tower.render_self(ctx),
                            }
                        })
                        .render(area, buf);
                }
                else {
                    Paragraph::new(text::Line::from(" dead ".bold().red()))
                        .centered()
                        .block(block.clone())
                        .render(area, buf);
                }
    }   
}

impl App {

    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<bool> {
        let time = Duration::from_micros(100);
        let mut wave = self.next_wave();
        let mut wave_complete = false;
        loop {
            if self.balloons.len() == 0 && wave_complete {
                wave = self.next_wave();
            }
            terminal.draw(|frame| self.render_frame(frame))?;
            if event::poll(time)? {
                self.handle_events().wrap_err("handle events failed")?;
            }
            if self.exit {
                break;
            }
            if self.restart {
                return Ok(false);
            }
            if self.on_pause || self.dead {
                continue;
            }
            wave_complete = self.handle_wave(&mut wave);
            self.move_wave()?;
            self.is_dead()?;
            self.generate_projectiles()?;
            self.handle_ballon_projectile_intereaction()?;
            self.highscore();
        }
        Ok(true)
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn highscore(&mut self) {
        if self.score > self.highscore {
            self.highscore = self.score;
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event).wrap_err_with(|| {
                    format!("handling key event failed: \n{key_event:#?}")
                })
            }
            Event::Mouse(mouse_event) => {
                self.handle_mouse_event(mouse_event).wrap_err_with(|| {
                    format!("handling mouse event failed: \n{mouse_event:#?}")
                })
            },
            Event::Resize(cols, rows) => self.handle_resize_event(cols, rows).wrap_err_with(|| {
                format!("handling mouse event failed: \n Resize event")
            }),
           _ => Ok(())
        }
    }

    pub fn new() -> Result<Self> {
        let (cols, rows) = size()?;
        let mut app = App {
            score: 0,
            highscore: 0,
            exit: false,
            dead: false,
            on_pause: false,
            path: BalloonPath::default(),
            balloons: vec![],
            towers: vec![],
            balloon_factory: BalloonFactory::default(),
            round: 0,
            max_cols: cols,
            max_rows: rows,
            gold: 30,
            hitpoints: 100,
            new_tower: None,
            tower_shop: TowerShop::new(),
            upgrade_shop_open: None,
            tower_shop_open: false,
            restart: false
        };
        app.path.generate_path();
        Ok(app)
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => self.pause()?,
            KeyCode::Enter => self.restart(),
            KeyCode::Right => {},
            _ => {}
        }
        Ok(())
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> Result<()> {
        let y = self.row_to_y(mouse_event.row);
        let x = self.col_to_x(mouse_event.column);
        match mouse_event.kind {
            MouseEventKind::Drag(MouseButton::Left) => {
                if let Some(tower) = self.new_tower.as_mut() {
                    tower.x = x;
                    tower.y = y;
                }
            }
            MouseEventKind::Up(MouseButton::Left) => {
                if y > -70.0 || !self.tower_shop_open {
                    if let Some(tower) = self.new_tower.as_ref() {
                        if !self.tower_on_path(tower) && !self.tower_collision(tower) {
                            self.towers.push(tower.clone());
                            self.gold -= self.towers[self.towers.len() - 1].cost;
                            self.new_tower = None;
                        }
                    }
                }
                else {
                    self.new_tower = None;
                }
            }
            MouseEventKind::Down(MouseButton::Left) => {
                if y <= -70.0 {
                    if self.tower_shop_open {
                        self.new_tower = self.tower_shop.get_tower(x, &self.gold);
                    }
                    else {
                        if y <= -83.0 && x <= -85.0 {
                            self.tower_shop_open = true;
                        }
                    }
                }
                else if y <= -65.0 && y >= -70.0 && x <= -85.0 && self.tower_shop_open{
                    self.tower_shop_open = false;
                }
                else if let Some(idx) = self.upgrade_shop_open {
                    if x >= 70.0 && (y >= -70.0 || !self.tower_shop_open) {
                        if let Some(cost) = self.towers[idx].buy_upgrade(y, &self.gold) {
                            self.gold -= cost;
                        }
                    }
                    else {
                        self.towers[idx].show_upgrades();
                        self.upgrade_shop_open = None;
                    }
                }
                else if let Some(idx) = self.mouse_over_tower(x, y) {
                    self.towers[idx].show_upgrades();
                    if self.towers[idx].upgrades.show_upgrades {
                        self.upgrade_shop_open = Some(idx)
                    }
                    else {
                        self.upgrade_shop_open = None;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_resize_event(&mut self, cols: u16, rows: u16) -> Result<()> {
        (self.max_cols, self.max_rows) = (cols, rows);
        Ok(())
    }

    fn restart(&mut self) {
        self.restart = true;
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn pause(&mut self) -> Result<()> {
        if self.on_pause {
            self.on_pause = false;
        }
        else {
            self.on_pause = true;
        }
        Ok(())
    }

    fn is_dead(&mut self) -> Result<()> {
        if !self.dead {
            if self.hitpoints <= 0 {
                self.dead = true;
            }
        }
        Ok(())
    }

    fn next_wave(&mut self) -> BalloonWave {
        self.round += 1;
        let wave = self.balloon_factory.generate_wave(self.round, self.path.elements[0].x, self.path.elements[0].y);
        wave
    }

    fn handle_wave(&mut self, wave: &mut BalloonWave) -> bool {
        if wave.ticks_since_last < wave.ticks_till_balloon {
            let _ = wave.next();
            return false;
        }
        let next_ballon = wave.next();
        match next_ballon {
            None => return true,
            Some(bloon) => {
                self.balloons.push(bloon);
            }
        }
        false
    }

    fn move_wave(&mut self) -> Result<()> {
        let mut k = 0;
        for i in 0..self.balloons.len() {
            if !self.balloons[i - k].move_balloon(&self.path)? {
                self.hitpoints -= self.balloons[i - k].damage;
                self.balloons.remove(i - k);
                k += 1;
            }
        }
        self.balloons.sort_by(|b1, b2| {
            b2.total_x.partial_cmp(&b1.total_x).unwrap()
        });
        Ok(())
    }

    fn row_to_y(&self, row: u16) -> f64 {
        let max = self.max_rows as f64;
        let mut y = row as f64 - max + 1.0; // range: (1.0?)0.0..max -> 0.0..1.0 -> 0.0..180.0 -> -90.0..90.0
        y /= -max;
        y *= 180.0;
        y -= 90.0;
        y
    }

    fn col_to_x(&self, col: u16) -> f64 {
        let max = self.max_cols as f64;
        let mut x = col as f64 + 1.0; // range: (1.0?)0.0..max -> 0.0..1.0 -> 0.0..180.0 -> -90.0..90.0
        x /= max;
        x *= 180.0;
        x -= 90.0;
        x
    }

    fn tower_on_path(&self, tower: &Tower) -> bool {
        self.path.point_on_path(tower)
    }

    fn mouse_over_tower(&self, x: f64, y: f64) -> Option<usize> {
        for (i, tower) in self.towers.iter().enumerate() {
            if (x >= tower.x && x <= tower.x + tower.width) && (y >= tower.y && y <= tower.y + tower.height) {
                return Some(i);
            }
        }
        None
    }

    fn tower_collision(&self, tower: &Tower) -> bool {
        self.towers.iter().any(|tower_| tower_.collides(tower)) || self.towers.iter().any(|tower_| tower.collides(tower_))
    }

    fn generate_projectiles(&mut self) -> Result<()> {
        for tower in self.towers.iter_mut() {
            tower.handle_projectile()?;
            if self.balloons.len() == 0 {
                continue;
            }
            let mut k = 0;
            for i in 0..self.balloons.len() {
                if self.balloons[i - k].is_dead() {
                    let (gold, score) = self.balloons[i - k].reward;
                    self.gold += gold;
                    self.score += score;
                    self.balloons.remove(i - k);
                    k += 1;
                    continue;
                }
                if tower.shoot(&self.balloons[i - k], &self.path, i - k)? {
                    break;
                }
            }
        }
        Ok(())
    }

    fn handle_ballon_projectile_intereaction(&mut self) -> Result<()> {
        for i in 0..self.balloons.len() {
            let dmg = self.damage_ballon(i)?;
            self.balloons[i].reduce_hitpoints(dmg);
            if self.balloons[i].is_dead() {
                for tower in self.towers.iter_mut() {
                    tower.remove_target_of_projectile(i)?;
                }
            }
        }
        
        Ok(())
    }

    fn damage_ballon(&mut self, index: usize) -> Result<f64> {
        let mut dmg = 0.0;
        for tower in self.towers.iter() {
            dmg += tower.calculate_damage(index);
        }
        Ok(dmg)
    }

}


#[derive(Debug, Default)]
pub struct BalloonPath {
    pub elements: Vec<RectangleInPath>
}

impl BalloonPath {
    fn generate_path(&mut self) {
        self.elements.push(RectangleInPath::horizontal(-90.0, 0.0, 0.0));
        self.elements.push(RectangleInPath::vertical(0.0, 40.0, 0.0));
        self.elements.push(RectangleInPath::horizontal(-45.0, 0.0, 30.0));
        self.elements.push(RectangleInPath::vertical(-10.0, 40.0, -45.0));
        self.elements.push(RectangleInPath::horizontal(-45.0, 90.0, -10.0));
    }

    fn point_on_path(&self, tower: &Tower) -> bool {
        self.elements.iter().map(|element| {
            element.point_on_self(tower)
        }).any(|x| x)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct RectangleInPath {
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub width: f64,
    pub is_horizontal: bool,
}

impl RectangleInPath {

    fn horizontal(x0: f64, x1: f64, y: f64) -> Self {
        RectangleInPath {
            x: x0,
            y: y,
            height: 10.0,
            width: x1 - x0,
            is_horizontal: true,
        }
    }

    fn vertical(y0: f64, y1: f64, x: f64) -> Self {
        RectangleInPath {
            x: x,
            y: y0,
            width: 5.0,
            height: y1 - y0,
            is_horizontal: false
        }
    }

    fn point_on_self(&self, tower: &Tower) -> bool {
        let x = tower.x;
        let y = tower.y;
        if self.is_horizontal {
            let y_check = (y >= self.y && y <= self.y + self.height) || (y + tower.height >= self.y && y + tower.height <= self.y + self.height);
            if self.width < 0.0 {
                return y_check && (x <= self.x && x >= self.x + self.width);
            }
            else {
                return y_check && (x >= self.x && x <= self.x + self.width);
            }
        }
        else {
            let x_check = (x >= self.x && x <= self.x + self.width) || (x + tower.width >= self.x && x + tower.height <= self.x + self.width);
            if self.height < 0.0 {
                return x_check && (y <= self.y && y >= self.y + self.height);
            }
            else {
                return x_check && (y >= self.y && y <= self.y + self.height);
            }
        }
    }

}