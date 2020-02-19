use ggez::*;
use ggez::event::EventHandler;
use ggez::graphics::{draw, Color, DrawParam};
use ggez::graphics::clear;
use ggez::graphics::present;
use graphics::Mesh;
use mint::Point2;


struct Circle {
    origin: Point2<f32>,
    radius: f32,
}

struct State {
    population: Vec<Circle>,
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, Color::from_rgb(120, 200, 110));

        for &x in &self.population {
            let mesh = Mesh::new_circle(ctx, graphics::DrawMode::fill(), x.origin, x.radius, 0.1, Color::from_rgb(230, 90, 90))?;
            draw(ctx, &mesh, DrawParam::default())?;
        }
        present(ctx)?;
        Ok(())
    }
}

//120 200 110 <- color for background
fn main() {
    let cb = ContextBuilder::new("ecosim", "Will Farris, Maggie Haddon");
    let (ref mut ctx, ref mut event_loop) = &mut cb
        .window_setup(ggez::conf::WindowSetup::default().title("Ecosystem Simulation"))
        .build()
        .unwrap();

    let mut shapes: Vec<Circle> = Vec::new();
    shapes.push(Circle { origin: Point2 {x: 200.0, y: 100.0}, radius: 30.0});
    shapes.push(Circle { origin: Point2 {x: 300.0, y: 200.0}, radius: 25.0});

    let state = &mut State {population: shapes};

    event::run(ctx, event_loop, state).unwrap();
}
