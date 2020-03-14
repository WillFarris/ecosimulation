//! Helper math functions for treating ggez's `Point2` data type as
//! a vector.
use ggez::mint::Point2;

/// Treating the given `Point2D` as a vector, returns its length
pub fn mag(vec: Point2<f32>) -> f32 {
    (vec.x * vec.x + vec.y * vec.y).sqrt()
}


/// Returns the euclidian distance between two `Point2D` structs a and b.
pub fn distance(p1: Point2<f32>, p2: Point2<f32>) -> f32 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    (dx * dx + dy * dy).sqrt()
}

#[test]
fn test_distance() {
    let a = Point2{x: 0.0, y: 0.0};
    let b = Point2{x: 5.0, y: 0.0};
    assert_eq!(5.0, distance(a, b));

    let c = Point2{x: 0.0, y: 0.0};
    let d = Point2{x: 0.0, y: 5.0};
    assert_eq!(5.0, distance(c, d));
}

/// Returns a normalized `Point2` representing the direction vector from `from` to `to`
pub fn point_to(from: Point2<f32>, to: Point2<f32>) -> Point2<f32> {
    normalize(
        Point2 {
            x: to.x - from.x,
            y: to.y - from.y,
        }
    )
}

/// Calculates the average vector between a and b
pub fn average_dir(a: Point2<f32>, b: Point2<f32>) -> Point2<f32> {
    Point2 {
        x: (a.x + b.x)/2.0,
        y: (a.y + b.y)/2.0,
    }
}

/// No implementation for multiplying `f32` with `Point2D`, so this function does that
pub fn scale_vec(scale: f32, vec: Point2<f32>) -> Point2<f32> {
    Point2 {
        x: vec.x * scale,
        y: vec.y * scale,
    }
}

/// Normalizes a `Point2D` to a unit vector
pub fn normalize(vec: Point2<f32>) -> Point2<f32> {
    let len = mag(vec);
    Point2 {
        x: vec.x / len,
        y: vec.y / len,
    }
}

#[test]
fn test_normalize() {
    let vec = normalize(Point2{x: 5.0, y: 5.0});
    let vec_mag = mag(vec);
    assert!(vec_mag > 0.99 && vec_mag < 1.01); //vec_mag == 1.0 fails for presumably floating point reasons
}
