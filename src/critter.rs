struct critter {
    x: u32,
    y: u32,
}

impl critter {
    fn mov(&mut self) {
        self.x += 1;
        self.y += 1;
    }
}