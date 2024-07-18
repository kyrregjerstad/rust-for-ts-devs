mod shapes;

use std::{fmt::Display, str::FromStr};

use crate::shapes::{circle::Circle, rect::Rect};

use anyhow::Result;
use shapes::collisions::{Collidable, Contains, Points};

enum Shape {
    Circle(Circle),
    Rect(Rect),
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (shape, data) = s.split_once(" ").unwrap_or(("", ""));

        match shape {
            "rect" => Ok(Shape::Rect(data.parse()?)),
            "circle" => Ok(Shape::Circle(data.parse()?)),
            _ => Err(anyhow::anyhow!("Unknown shape")),
        }
    }
}

impl Points for &Shape {
    fn points(&self) -> shapes::collisions::PointIter {
        match self {
            Shape::Circle(c) => c.points(),
            Shape::Rect(r) => r.points(),
        }
    }
}

impl Contains for &Shape {
    fn contains_point(&self, point: (f64, f64)) -> bool {
        match self {
            Shape::Circle(c) => c.contains_point(point),
            Shape::Rect(r) => r.contains_point(point),
        }
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Circle(c) => write!(f, "circle {}", c),
            Shape::Rect(r) => write!(f, "rect {}", r),
        }
    }
}

fn main() -> Result<()> {
    let shapes = std::fs::read_to_string("shapes")?
        .lines()
        .filter_map(|x| x.parse::<Shape>().ok())
        .collect::<Vec<_>>();

    shapes
        .iter()
        .skip(1)
        .zip(shapes.iter().take(shapes.len() - 1))
        .filter(|(a, b)| a.collide(b))
        .for_each(|(a, b)| println!("{} collides with {}", a, b));

    return Ok(());
}
