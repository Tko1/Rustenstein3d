use std::fmt;

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

#[derive(Copy,Clone)]
pub struct Angle(pub Vec2f);

impl Angle {
    pub fn new(x:f32, y:f32) -> Angle { Angle(Vec2f::new(x,y)) }
    pub fn get_vec(&self) -> Vec2f { self.0 }
    pub fn get_rad(&self) -> f32
    {
        let Vec2f {x, y} = self.get_vec();
        return y.atan2(x);
    }
    pub fn slope(&self) -> f32 {
        let vec = self.get_vec();
        return vec.y / vec.x;
    }
    pub fn write(&self,f: &mut fmt::Formatter) -> fmt::Result {
	// Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
	write!(f, "Angle{{{:?}}}[As Rad?: {} , As Degrees: {}]", self.0,self.get_rad(),self.get_rad().to_degrees())
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

impl fmt::Display for Angle {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	self.write(f)
    }
}

impl fmt::Debug for Angle {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	self.write(f)
    }
}

