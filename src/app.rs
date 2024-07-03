use crate::tui;

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
use crate::balloons::Ballon;

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
    towers: Vec<Tower>
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
            self.highscore();
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
            towers: vec![]
        };
        app.path.generate_path();
        Ok(app)
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => self.pause()?,
            KeyCode::Enter => self.restart()?,
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
}


#[derive(Debug, Default)]
struct BallonPath {
    elements: Vec<RectangleInPath>
}

impl BallonPath {
    fn generate_path(&mut self) {
        for i in 0..180 {
            self.elements.push(RectangleInPath::new((i * 10).to_f64().unwrap() - 90.0, 0.0));
        }
    }
}

#[derive(Debug, Default)]
struct RectangleInPath {
    x: f64,
    y: f64,
    height: f64,
    width: f64
}

impl RectangleInPath {
    fn new(x: f64, y: f64) -> Self {
        RectangleInPath {
            x: x,
            y: y,
            height: 5.0,
            width: 10.0
        }
    }
}