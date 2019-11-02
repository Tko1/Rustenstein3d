use crate::math::Vec2f;
use crate::math::Angle;

pub struct Transform(pub Vec2f);
 
struct Camera {
    transform: Transform,
    rotation: Angle,
    horizontal_view_angle: Angle,
    vertical_view_angle: Angle,
    //Basically, how many pixels wide this view is, and therefore how many rays we need to cast
    view_width: i32
}
