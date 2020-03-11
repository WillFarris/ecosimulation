pub mod critters {
    use crate::math::math::{anglebetween, distance};
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
        speed: f32,        // faster creatures consume energy more quickly
        pub eyesight: f32, // creatures with better eyesight can sense food further away

        pub max_hunger: f32, // depends on size

        // "Lifetime" traits
        pub hunger: f32, //decreases with movement, creature dies if it starves to death
        pub is_dead: bool, //dead flag, all dead creatures are removed each update() tick
        pub wants_mate: bool, //horny flag, signals whether or not the critter wants to mate
    }

    impl Prey {
        pub fn new() -> Self {
            let mut rng = thread_rng();
            let angle: f32 = rng.gen_range(0.0, 6.28318530718);
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
                size: size,
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
            //pink if horny (can change back, mostly to test timer)
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
                    let current_distance = distance(&self.position, &f.position) - f.size;
                    if current_distance < nearest_distance {
                        nearest_food = Some(f);
                        nearest_distance = current_distance;
                    }
                }
            }

            let target_food = nearest_food.unwrap();

            if nearest_distance < self.eyesight {
                let angle = anglebetween(&self.position, &target_food.position);
                self.direction.x = angle.cos();
                self.direction.y = angle.sin();
            }

            if nearest_distance < self.size {
                self.hunger += 10.0 * target_food.sustenance;
                if self.hunger > MAX_HUNGER {
                    self.hunger = MAX_HUNGER;
                }
                target_food.consumed = true;
            }
        }

        pub fn look_at(&mut self, angle: f32) {
            self.direction.x = angle.cos();
            self.direction.y = angle.sin();
        }
    }

    pub fn mate_prey(a: &Prey, b: &Prey) -> Prey {
        let mut rng = thread_rng();
        let mut my_size = a.size;
        if a.size > b.size {
            my_size = rng.gen_range(b.size * 0.75, a.size * 1.25);
        } else if b.size < a.size {
            my_size = rng.gen_range(a.size * 0.75, b.size * 1.25);
        }

        let mut my_speed = b.speed;
        if a.speed > b.speed {
            my_speed = rng.gen_range(b.speed * 0.75, a.speed * 1.25);
        } else if b.speed < a.speed {
            my_speed = rng.gen_range(a.speed * 0.75, b.speed * 1.25);
        }
        if (my_speed * 10.0) as i32 % 6 == 0 {
            my_speed *= 10.0;
        } //turbocritter

        let mut my_sight = a.eyesight;
        if a.eyesight > b.eyesight {
            my_sight = rng.gen_range(b.eyesight * 0.75, a.eyesight * 1.25);
        } else if b.eyesight < a.eyesight {
            my_sight = rng.gen_range(a.eyesight * 0.75, b.eyesight * 1.25);
        }
        Prey {
            position: a.position,
            color: Color::from_rgb(0, 0, 255),
            direction: Point2 { x: 1.0, y: 0.0 },
            size: my_size,
            speed: my_speed,
            eyesight: my_sight,
            max_hunger: my_size * 100 as f32,
            hunger: MAX_HUNGER,
            is_dead: false,
            wants_mate: true,
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
            Self {
                position: Point2 {
                    x: rng.gen_range(0.0, 800.0),
                    y: rng.gen_range(0.0, 600.0),
                },
                size: size,
                cur_size: size,
                color: Color::from_rgb(40, 90, 30),
                sustenance: size * 2.0,
                consumed: false,
            }
        }

        pub fn update(&mut self) {
            if self.consumed {
                //self.color = Color::from_rgb(252, 186, 3);
                self.color = Color::from_rgb(120, 200, 110);
                self.cur_size = 2.0;
            }
        }

        pub fn grow(&mut self) {
            if !self.consumed && self.cur_size < self.size {
                self.cur_size += 2.0;
            }
            if self.consumed {
                self.consumed = false;
                self.color = Color::from_rgb(40, 90, 30);
            }
        }
    }
}
