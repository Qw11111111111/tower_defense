use crate::{ballons::BallonFactory, tui};

use canvas::{Canvas, Circle, Rectangle};
use color_eyre::{
    eyre::WrapErr, owo_colors::OwoColorize, Result
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use num::ToPrimitive;
use ratatui::{
    prelude::*, 
    style::Color, 
    widgets::{block::{Position, *}, Paragraph, *}
};

use std::path::Path;

use crate::read_write::*;

use crate::towers::*;
use crate::ballons::Ballon;

#[derive(Debug, Default)]
pub struct App {
    pub score: u64,
    pub highscore: u64,
    exit: bool,
    on_pause: bool,
    dead: bool,
    won: bool,
    path: BallonPath,
    ballons: Vec<Ballon>,
    towers: Vec<Tower>,
    ballon_factory: BallonFactory,
    round: usize
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
        where
            Self: Sized {

                let instructions = Title::from(Line::from(vec![
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

                Paragraph::new(Line::from(self.score.to_string()))
                    .alignment(Alignment::Left)
                    .block(block.clone())
                    .render(area, buf);

                Paragraph::new(Line::from(self.highscore.to_string()))
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
                            for rect in self.path.elements.iter() { // draw the path of the ballons
                                ctx.draw(&Rectangle {
                                    x: rect.x,
                                    y: rect.y,
                                    height: rect.height,
                                    width: rect.width,
                                    color: Color::White
                                })
                            }
                            ctx.layer();
                            for ballon in self.ballons.iter() { // draw the ballons
                                ctx.draw(&Circle {
                                    x: ballon.x,
                                    y: ballon.y,
                                    radius: ballon.radius,
                                    color: ballon.color
                                })
                            }
                            ctx.layer();
                            for tower in self.towers.iter() { // draw the towers
                                ctx.draw(&Rectangle {
                                    x: tower.x,
                                    y: tower.y,
                                    width: tower.width,
                                    height: tower.height,
                                    color: tower.color
                                })
                            }
                            ctx.layer();
                            for tower in self.towers.iter() { // draw all projectiles
                                for projectile in tower.projectiles.iter() {
                                    ctx.draw(&Circle {
                                        x: projectile.x,
                                        y: projectile.y,
                                        radius: projectile.radius,
                                        color: projectile.color
                                    })
                                }
                            }
                        })
                        .render(area, buf);
                }
                else {
                    Paragraph::new(Line::from(" dead ".bold().red()))
                        .centered()
                        .block(block.clone())
                        .render(area, buf);
                }

                if self.won {
                    Paragraph::new(Line::from(vec![Span::from(" Congratulations you won |".bold()), Span::from(" restart: <Enter>, continue: <c>".bold())]))
                        .centered()
                        .block(block.clone())
                        .render(area, buf);
                }
    }   
}

impl App {

    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
            if self.exit {
                break;
            } 
            if self.on_pause || self.dead {
                continue;
            }
            self.move_wave();
            self.highscore();
            self.handle_wave();
        }
        Ok(())
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
           _ => Ok(())
        }
    }

    pub fn new() -> Result<Self> {
        let mut app = App {
            score: 0,
            highscore: 0,
            exit: false,
            dead: false,
            on_pause: false,
            won: false,
            path: BallonPath::default(),
            ballons: vec![],
            towers: vec![],
            ballon_factory: BallonFactory::default(),
            round: 1
        };
        app.path.generate_path();
        Ok(app)
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => self.pause()?,
            KeyCode::Enter => self.restart()?,
            KeyCode::Right => {},
            _ => {}
        }
        Ok(())
    }

    fn restart(&mut self) -> Result<()> {

        if self.dead {
            let path = Path::new("Highscore.bin");
            save(path, self.highscore)?;
            
            let num = read(path)?;

            self.highscore = num;
            self.score = 0;
            self.on_pause = false;
            self.dead = false;
        }

        Ok(())
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
            self.dead = true;
        }
        Ok(())
    }

    fn next_wave(&mut self) {
        self.ballons = self.ballon_factory.generate_wave(self.round, self.path.elements[0].x, self.path.elements[0].y);
    }

    fn handle_wave(&mut self) {
        if self.ballons.len() == 0 {
            self.next_wave();
        }
    }

    fn move_wave(&mut self) {
        let mut k = 0;
        for i in 0..self.ballons.len() {
            let boolena = self.ballons[i - k].move_ballon(&self.path);
            if !boolena {
                self.ballons.remove(i - k);
                k += 1;
            }
        }
    }

}


#[derive(Debug, Default)]
pub struct BallonPath {
    pub elements: Vec<RectangleInPath>
}

impl BallonPath {
    fn generate_path(&mut self) {
        self.elements.push(RectangleInPath::horizontal(-90.0, 0.0, 0.0));
        self.elements.push(RectangleInPath::vertical(0.0, 30.0, 0.0));
        self.elements.push(RectangleInPath::horizontal(0.0, 90.0, 30.0));
    }
}

#[derive(Debug, Default)]
pub struct RectangleInPath {
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub width: f64,
    pub is_horizontal: bool,
}

impl RectangleInPath {
    fn new(x: f64, y: f64) -> Self {
        RectangleInPath {
            x: x,
            y: y,
            height: 10.0,
            width: 10.0,
            is_horizontal: true,
        }
    }

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
}