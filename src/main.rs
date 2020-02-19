use ggez::*;
use ggez::event::EventHandler;
use ggez::conf::WindowMode;

struct State {}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {


        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {


        Ok(())
    }
}

//120 200 110 <- color for background
fn main() {
    let state = &mut State {};

    let cb = ContextBuilder::new("ecosim", "Will Farris, Maggie Haddon");
    let (ref mut ctx, ref mut event_loop) = &mut cb
        .window_setup(ggez::conf::WindowSetup::default().title("Ecosystem Simulation"))
        .build()
        .unwrap();

    event::run(ctx, event_loop, state).unwrap();
}
