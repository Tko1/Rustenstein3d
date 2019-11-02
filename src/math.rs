#[derive(Debug,Copy,Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

pub type Vec2f = Vec2<f32>;

impl<T> Vec2<T> {
    pub fn new(x : T,y : T) -> Vec2<T> {
        Vec2 {x: x, y: y}
    }
} 
