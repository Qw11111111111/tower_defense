pub mod errors;
pub mod tui;
pub mod app;
pub mod read_write;
pub mod towers;
pub mod ballons;
pub mod utils;

use {
    app::App,
    read_write::*,
    std::{
        fs::File,
        path::Path
    },
    color_eyre::Result
};

fn main() -> Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    let path = Path::new("Highscore.bin");
    let number: u64;
    if !path.exists() {
        File::create(path)?;
        number = 0;
    }
    else {
        number = read(&path)?;
    }

    let mut app = App::new()?;
    app.highscore = number;
    app.run(&mut terminal)?;
    tui::restore()?;
    
    save(path, app.highscore)?;
    Ok(())
}

