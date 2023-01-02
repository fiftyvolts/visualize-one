#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z}
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {r, g, b}
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub point: Point,
    pub color: Color
}

impl Vertex {
    pub fn new(point: Point, color: Color) -> Self {
        Self {point, color }
    }
}
