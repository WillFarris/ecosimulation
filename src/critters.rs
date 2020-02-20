pub mod critters {
    use ggez::mint::Point2;
    use ggez::graphics::Color;
    use rand::{thread_rng, Rng};

    pub struct Prey {
        pub position: Point2<f32>,
        pub size: f32,
        pub color: Color,
        direction: Point2<f32>,
        speed: f32,
    }

    impl Prey {
        pub fn new() -> Self {
            let mut rng = thread_rng();
            let angle: f32 = rng.gen_range(0.0, 6.28318530718);

            Prey {
                position: Point2 {x: rng.gen_range(0.0, 800.0), y: rng.gen_range(0.0, 600.0)},
                direction: Point2 {x: angle.cos(), y: angle.sin()},
                speed: rng.gen_range(1.0, 2.0),
                size: 10.0,
                color: Color::from_rgb(200, 50, 90),
            }
        }

        pub fn update(&mut self) {
            if self.position.x + self.size > 800.0 || self.position.x - self.size < 0.0 {
                self.direction.x *= -1.0;
                self.speed += 1.0;
            }
            if self.position.y + self.size > 600.0 || self.position.y - self.size < 0.0 {
                self.direction.y *= -1.0;
            }
            self.position.x += self.direction.x * self.speed;
            self.position.y += self.direction.y * self.speed;
        }
    }

    pub struct Food {
        pub position: Point2<f32>,
        pub size: f32,
        pub color: Color,
    }

    impl Food {
        pub(crate) fn new() -> Self {
            let mut rng = thread_rng();
            Self {
                position: Point2 {x: rng.gen_range(0.0, 800.0), y: rng.gen_range(0.0, 600.0)},
                size: rng.gen_range(1.0, 7.0),
                color: Color::from_rgb(40, 90, 30),
            }
        }
    }
}