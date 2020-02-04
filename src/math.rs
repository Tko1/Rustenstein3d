extern crate num;

use std::fmt;
use std::ops;
use num::Integer;
use num::Float;

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

impl Vec2f {
    /*
    Make no mistake,  this is more for position like Vec2fs, but I don't want 
    to wrap everything in a newtype, I have found in Haskell in trying to 
    take type wrapping and type power to its extreme that sometimes in short 
    scoped projects, its just not worth it
     */
    pub fn distance(&self,other: &Vec2f) -> f32
    {
	let dx = (other.x - self.x).abs();
	let dy = (other.y - self.y).abs();
	// TODO: sometimes I'm ending with expressions, sometimes with
	// return statements. Pick a side 
	(dx.powi(2) + dy.powi(2)).sqrt()
    }
    pub fn magnitude(&self) -> f32
    {
	(self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn of_magnitude(&self,magnitude: f32) -> Vec2f
    {
	let factor = magnitude / self.magnitude();
	Vec2f::new(self.x * factor, self.y * factor)
    }
    pub fn forward(&self) -> Vec2f
    {
	self.of_magnitude(1.0)
    }
    pub fn rotate(&self,angle: f32) -> Vec2f {
	let x = if self.x == 0.0 { 0.00001 } else { self.x };
	let y = if self.y == 0.0 { 0.00001 } else { self.y };
	Vec2f::new(
	    x * angle.cos() - y * angle.sin(),
	    x * angle.sin() + y * angle.cos()
	)
    }
}

impl ops::Add<Vec2f> for Vec2f {
    type Output = Vec2f;
    fn add(self,_rhs: Vec2f) -> Vec2f {
	Vec2f::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

impl ops::Mul<f32> for Vec2f {
    type Output = Vec2f;
    fn mul(self,_rhs: f32) -> Vec2f {
	Vec2f::new(self.x * _rhs, self.y * _rhs)
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
    pub fn rotate(&self,angle: f32) -> Angle {
	Angle(self.0.rotate(angle))
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
