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

pub fn average_dir(a: Point2<f32>, b: Point2<f32>) -> Point2<f32> {
    Point2 {
        x: (a.x + b.x)/2.0,
        y: (a.y + b.y)/2.0,
    }
}

pub fn reverse_dir(vec: Point2<f32>) -> Point2<f32> {
    Point2 {
        x: vec.x * -1.0,
        y: vec.y * -1.0,
    }
}

pub fn normalize(vec: Point2<f32>) -> Point2<f32> {
    let len = (vec.x * vec.x + vec.y * vec.y).sqrt();
    Point2 {
        x: vec.x / len,
        y: vec.y / len,
    }
}
