use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::{clear, draw, present, Color, DrawParam, Mesh, Rect};
use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;
mod critters;
mod math;
use crate::math::math::distance;
use critters::critters::*;

struct GameState {
    population: Vec<Prey>,
    new_children: Vec<Prey>,
    food: Vec<Food>,
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> Self {
        let mut population = Vec::new();
        let mut food = Vec::new();
        for _i in 0..5 {
            population.push(Prey::new());
        }
        for _i in 0..20 {
            food.push(Food::new());
        }
        GameState {
            population,
            new_children: vec![],
            food,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for i in &mut self.population {
            i.seek_food(&mut self.food);
            i.update();
            i.mate_prey(&mut self.new_children);
        }
        self.population.append(&mut self.new_children);
        self.population.retain(|x| !x.is_dead);
        self.food.retain(|x| !x.consumed);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, Color::from_rgb(120, 200, 110));

        for f in &self.food {
            let mesh = Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                f.position,
                f.size,
                0.1,
                f.color,
            )?;
            draw(ctx, &mesh, DrawParam::default())?;
        }

        for p in &self.population {
            let eyesight_mesh = Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                p.position,
                p.eyesight,
                0.1,
                Color::from_rgba(255, 0, 0, 50),
            )?;
            let body_mesh = Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                p.position,
                p.size,
                0.1,
                p.color,
            )?;
            let hunger_bg = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect {
                    x: p.position.x - (3.0 * p.size),
                    y: p.position.y - (3.0 * p.size),
                    w: (6.0 * p.size),
                    h: 10.0,
                },
                Color::from_rgb(100, 100, 100),
            )?;
            let hunger_bar = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect {
                    x: p.position.x - (3.0 * p.size),
                    y: p.position.y - (3.0 * p.size),
                    w: (6.0 * p.size) * (p.hunger / p.max_hunger),
                    h: 10.0,
                },
                Color::from_rgb(255, 0, 0),
            )?;
            draw(ctx, &eyesight_mesh, DrawParam::default())?;
            draw(ctx, &body_mesh, DrawParam::default())?;
            draw(ctx, &hunger_bg, DrawParam::default())?;
            draw(ctx, &hunger_bar, DrawParam::default())?;
        }

        present(ctx)?;
        Ok(())
    }
}

fn main() {
    let (mut ctx, mut event_loop) =
        ContextBuilder::new("Ecosystem Simulation", "Will Farris, Maggie Haddon")
            .window_setup(ggez::conf::WindowSetup::default().title("Ecosystem Simulation"))
            .build()
            .expect("Could not create ggez context!");

    let mut game_state = GameState::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut game_state) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error: {}", e),
    }
}
