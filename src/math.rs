    use ggez::mint::Point2;

    pub fn distance(p1: Point2<f32>, p2: Point2<f32>) -> f32 {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn anglebetween(from: Point2<f32>, to: Point2<f32>) -> f32 {
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        dy.atan2(dx)
    }
