use crate::math::*;
use ggez::graphics::Color;
use ggez::mint::Point2;
use rand::{thread_rng, Rng};

pub const MAX_HUNGER: f32 = 500.0;

pub struct Prey {
    // Non-genetic traits, mostly for drawing and graphics
    pub position: Point2<f32>,
    pub color: Color,
    pub direction: Point2<f32>,

    // Genetic traits
    pub size: f32,     // larger creatures can go longer without food
    pub speed: f32,    // faster creatures consume energy more quickly
    pub eyesight: f32, // creatures with better eyesight can sense food further away

    pub max_hunger: f32, // depends on size

    // "Lifetime" traits
    pub hunger: f32,   //decreases with movement, creature dies if it starves to death
    pub is_dead: bool, //dead flag, all dead creatures are removed each update() tick
    pub wants_mate: bool, //signals whether or not the critter wants to mate
}

impl Prey {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let angle: f32 = rng.gen_range(0.0, std::f32::consts::PI);
        let size: f32 = rng.gen_range(5.0, 10.0);

        Prey {
            position: Point2 {
                x: rng.gen_range(0.0, 800.0),
                y: rng.gen_range(0.0, 600.0),
            },
            direction: Point2 {
                x: angle.cos(),
                y: angle.sin(),
            },
            color: Color::from_rgb(200, 50, 90),

            speed: rng.gen_range(0.1, 0.5),
            size,
            max_hunger: size * 100.0,
            eyesight: rng.gen_range(size + 10.0, size + 50.0),

            hunger: MAX_HUNGER,
            is_dead: false,
            wants_mate: true,
        }
    }

    pub fn update(&mut self) {
        if self.hunger > 0.0 {
            if self.position.x + self.size > 800.0 || self.position.x - self.size < 0.0 {
                self.direction.x *= -1.0;
            }
            if self.position.y + self.size > 600.0 || self.position.y - self.size < 0.0 {
                self.direction.y *= -1.0;
            }
            self.position.x += self.direction.x * self.speed;
            self.position.y += self.direction.y * self.speed;

            self.hunger -= self.speed;
        } else {
            self.is_dead = true;
        }
        // pink indicates this Prey is looking for a mate
        if self.wants_mate {
            self.color = Color::from_rgb(247, 106, 210);
        } else {
            self.color = Color::from_rgb(200, 50, 90);
        }
    }

    pub fn seek_food(&mut self, food_vec: &mut Vec<Food>) {
        if food_vec.is_empty() {
            return;
        }

        let mut nearest_food = None;
        let mut nearest_distance: f32 = std::f32::INFINITY;
        for f in food_vec {
            if !f.consumed {
                let current_distance = distance(self.position, f.position) - f.cur_size;
                if current_distance < nearest_distance {
                    nearest_food = Some(f);
                    nearest_distance = current_distance;
                }
            }
        }
        if nearest_food.is_none() {
            return;
        }

        let target_food = nearest_food.unwrap();

        if nearest_distance < self.eyesight {
            self.direction = point_to(self.position, target_food.position);
        }

        if nearest_distance < self.size {
            self.hunger += 10.0 * target_food.sustenance;
            if self.hunger > MAX_HUNGER {
                self.hunger = MAX_HUNGER;
            }
            target_food.consumed = true;
        }
    }

    pub fn look_at(&mut self, towards: Point2<f32>) {
        self.direction = point_to(self.position, towards);
    }
}

pub fn mate_prey(a: &Prey, b: &Prey) -> Prey {
    let mut rng = thread_rng();
    let mut my_size = if rng.gen_bool(0.5) {a.size} else {b.size};
    let mut my_speed = if rng.gen_bool(0.5) {a.speed} else {b.speed};
    let mut my_sight = if rng.gen_bool(0.5) {a.eyesight} else {b.eyesight};

    // random change at 10% mutation for each gene
    if rng.gen_ratio(1, 20) {
        my_size *= rng.gen_range(0.9, 1.1);
    }
    if rng.gen_ratio(1, 20) {
        my_speed *= rng.gen_range(0.9, 1.1);
    }
    if rng.gen_ratio(1, 20) {
        my_sight *= rng.gen_range(0.9, 1.1);
    }

    let my_direction = scale_vec(-1.0, normalize(average_dir(a.position, b.position)));

    Prey {
        position: a.position,
        color: Color::from_rgb(0, 0, 255),
        direction: my_direction,
        size: my_size,
        speed: my_speed,
        eyesight: my_sight,
        max_hunger: MAX_HUNGER,
        hunger: MAX_HUNGER,
        is_dead: false,
        wants_mate: false,
    }
}

pub struct Food {
    pub position: Point2<f32>,
    pub size: f32,
    pub cur_size: f32,
    pub color: Color,
    pub sustenance: f32,
    pub consumed: bool,
}

impl Food {
    pub(crate) fn new() -> Self {
        let mut rng = thread_rng();
        let size = rng.gen_range(5.0, 20.0);
        let size_diff = rng.gen_range(0.0, 4.0);
        Self {
            position: Point2 {
                x: rng.gen_range(0.0, 800.0),
                y: rng.gen_range(0.0, 600.0),
            },
            size,
            cur_size: size - size_diff,
            color: Color::from_rgb(40, 90, 30),
            sustenance: size * 2.0,
            consumed: false,
        }
    }

    pub fn update(&mut self) {
        if self.consumed {
            self.color = Color::from_rgb(120, 200, 110);
            self.cur_size = 2.0;
        }
    }

    pub fn grow(&mut self) {
        if !self.consumed && self.cur_size < self.size {
            self.cur_size += 2.0;
            self.sustenance = self.cur_size * 2.0
        }
        if self.consumed {
            self.consumed = false;
            self.color = Color::from_rgb(40, 90, 30);
            self.sustenance = self.cur_size * 2.0
        }
    }
}
