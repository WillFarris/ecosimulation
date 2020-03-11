use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::{clear, draw, present, Color, DrawParam, Mesh, Rect};
use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;
use std::time::Duration;
use std::time::Instant;
mod critters;
mod math;
use crate::math::math::{anglebetween, distance};
use critters::critters::*;

struct GameState {
    population: Vec<Prey>,
    food: Vec<Food>,
    dt: Duration,
    start_time: Instant,
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> Self {
        let mut population = Vec::new();
        let mut food = Vec::new();
        let start_time = Instant::now();
        let dt = Duration::new(0, 0);
        for _i in 0..10 {
            population.push(Prey::new());
        }
        for _i in 0..30 {
            food.push(Food::new());
        }
        GameState {
            population,
            food,
            dt,
            start_time,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for i in 0..self.population.len() {
            for j in 0..self.population.len() {
                if i != j && self.population[i].wants_mate && self.population[j].wants_mate {
                    let dist = distance(&self.population[i].position, &self.population[j].position);
                    if dist < self.population[i].size + self.population[j].size {
                        self.population
                            .push(mate_prey(&self.population[i], &self.population[j]));
                        self.population[i].wants_mate = false;
                        self.population[j].wants_mate = false;
                    } else if dist < self.population[i].eyesight + self.population[j].eyesight {
                        let to_j = anglebetween(
                            &self.population[i].position,
                            &self.population[j].position,
                        );
                        self.population[i].look_at(to_j);
                        self.population[j].look_at(to_j - 3.141592);
                    }
                }
            }
            self.population[i].seek_food(&mut self.food);
            self.population[i].update();

            //mating season
            let cur_time = Instant::now();
            self.dt = cur_time.duration_since(self.start_time);
            let tmate = self.dt.as_millis() as u32;
            if tmate % 4000 == 1 {
                self.population[i].wants_mate = true;
            }
        }
        self.population.retain(|x| !x.is_dead);
        //self.food.retain(|x| !x.consumed);

        for j in 0..self.food.len() {
            let cur_time = Instant::now();
            self.dt = cur_time.duration_since(self.start_time);
            let t = self.dt.as_millis() as u32;
            self.food[j].update();
            if t % 200 == 1 {
                self.food[j].grow();
            }
        }
        /*if self.food.len() < 20 {
            for _i in 0..3 {
                self.food.push(Food::new());
            }
        }
        */
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, Color::from_rgb(120, 200, 110));

        for f in &self.food {
            let mesh = Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                f.position,
                f.cur_size,
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
