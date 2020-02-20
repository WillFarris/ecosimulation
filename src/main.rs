use ggez::event::EventHandler;
use ggez::graphics;
use ggez::ContextBuilder;
use ggez::graphics::{draw, Color, DrawParam, clear, present, Mesh};
use ggez::event;
use ggez::Context;
use ggez::GameResult;
mod critters;
use critters::critters::*;

struct State {
    population: Vec<Prey>,
    food: Vec<Food>,
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for i in &mut self.population {
            i.update();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, Color::from_rgb(120, 200, 110));


        for f in &self.food {
            let mesh = Mesh::new_circle(ctx, graphics::DrawMode::fill(), f.position, f.size, 0.1, f.color)?;
            draw(ctx, &mesh, DrawParam::default())?;
        }

        for p in &self.population {
            let mesh = Mesh::new_circle(ctx, graphics::DrawMode::fill(), p.position, p.size, 0.1, p.color)?;
            draw(ctx, &mesh, DrawParam::default())?;
        }

        present(ctx)?;
        Ok(())
    }
}

fn main() {
    let cb = ContextBuilder::new("ecosim", "Will Farris, Maggie Haddon");
    let (ref mut ctx, ref mut event_loop) = &mut cb
        .window_setup(ggez::conf::WindowSetup::default().title("Ecosystem Simulation"))
        .build()
        .unwrap();

    let mut population: Vec<Prey> = Vec::new();
    for _i in 0..5 {
        population.push(Prey::new());
    }

    let mut food: Vec<Food> = Vec::new();
    for _i in 0..10 {
        food.push(Food::new());
    }

    let state = &mut State {population, food};

    event::run(ctx, event_loop, state).unwrap();
}
