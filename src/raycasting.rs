use crate::math::Vec2f;

#[derive(Debug,Copy,Clone)]
pub struct Angle(pub Vec2f);

impl Angle {
    pub fn new(x:f32, y:f32) -> Angle { Angle(Vec2f::new(x,y)) }
    pub fn get_vec(&self) -> Vec2f { self.0 }
    pub fn get_rad(&self) -> f32
    {
        let Vec2f {x: x, y: y} = self.get_vec();
        return y.atan2(x);
    }
}

pub trait ToAngle {
    fn to_angle(&self) -> Angle;
}
impl ToAngle for f32 {
    fn to_angle(&self) -> Angle {
        Angle(Vec2f::new(self.cos(),self.sin()))      
    }
}

