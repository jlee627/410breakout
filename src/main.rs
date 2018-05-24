/*
Main file for Breakout game using the ggez engine.

NOTE: Initial main file is mostly a copy of the Hello_World example
from the ggez site.
*/

extern crate ggez;

use ggez::conf;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::graphics;
//use ggez::timer;

use std::env;
use std::path;

// Structure to contain the game's state
struct MainState {
    text: graphics::Text,
    frames: usize,
}

impl MainState {
    fn new (ctx: &mut Context) -> GameResult<MainState> {
        // The ttf file will be in your resources directory
        // Which will be mounted later, so no need to path it here

        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf",48)?;
        let text = graphics::Text::new(ctx, "Hello world!", &font)?;

        let s = MainState {
            text,
            frames:0,
            };
    };
        Ok(s)
}

impl event::EventHandler for MainState {
        fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
            Ok(())
        }

        fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
            graphics::clear(ctx);

            // Drawables are drawn from their top-left corner

            let dest_point = graphics::Point2::new(10.0, 10.0);
            graphics::draw(ctx, &self.text, dest_point, 0.0)?;
            graphics::present(ctx);

            self.frames += 1;

            if (self.frames % 100) == 0 {
                println!("FPS: {}", ggez::timer::get_fps(ctx));
            }
            Ok(())
        }
    }
    
    // Now our main function, which does three things:
    //
    // * First, create a new `ggez::conf::Conf`
    // object which contains configuration info on things such
    // as screen resolution and window title.
    // * Second, create a `ggez::game::Game` object which will
    // do the work of creating our MainState and running our game.
    // * Then, just call `game.run()` which runs the `Game` mainloop.
fn main(){
    // Window settings
    // Game settings
    // Configuration Settings
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("helloworld", "ggez", c).unwrap();

    // We add the CARGO_MANIFEST_DIR/resources to the filesystem's path
    // so that ggez will look in our cargo project directory for files.

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
           let mut path = path::PathBuf::from(manifest_dir);
           path.push("resources");
           ctx.filesystem.mount(&path, true);
       }

       let state = &mut MainState::new(ctx).unwrap();

       if let Err(e) = event::run(ctx, state) {
           println!("Error encountered: {}", e);
       } else {
           println!("Game exited cleanly.");
       }


}
