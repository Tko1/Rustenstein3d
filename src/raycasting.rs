use crate::math::Vec2f;
use crate::math::Angle;
use crate::math::ToAngle;

const DEFAULT_VIEW_WIDTH : i32 = 5;

enum MapEntity {
    Wall,
    Floor,
    Enemy,
    Player
}

/// TODO Wrap in newtype?
/// For now,  would like to get a prototype up and running
/// and I want all the 2d vector syntax automatically for now
type Map = Vec<Vec<MapEntity>>;

pub struct Transform(pub Vec2f);
impl Transform {
    fn new(x:f32,y:f32) -> Transform {
        Transform(Vec2f::new(x,y))
    }
}
 
pub struct Camera {
    pub transform: Transform,
    pub rotation: Angle,
    pub horizontal_view_angle: Angle,
    pub vertical_view_angle: Angle,
    // Basically, how many pixels wide this view is, and therefore how many rays we need to cast
    pub view_width: i32
}
impl Default for Camera {
    fn default() -> Camera {
        Camera {
            transform: Transform::new(0.0,0.0),
            rotation: Angle::new(0.0,0.0),
            horizontal_view_angle: Angle::new(-1.0,1.0),
            vertical_view_angle: Angle::new(-1.0,1.0),
            view_width: DEFAULT_VIEW_WIDTH
        }
    }
}
impl Camera {
    /// For now,  everything will be thrown in here before being divvied up
    /// See documentation at docs/raycast.org 
    pub fn raycast(&self) {
        let view_ang_rad = self.horizontal_view_angle.get_rad();
        let rot_rad = self.rotation.get_rad();
        assert!(view_ang_rad >= 0.0 && rot_rad >= 0.0);

        // The ray angle pointing from our leftmost line of vision 
        let view_rad_right = rot_rad - (view_ang_rad / 2.0);
        // The ray angle pointing from our rightmost line of vision
        let view_rad_left = rot_rad + (view_ang_rad / 2.0);

        let ray_count = self.view_width;
        let ray_angles = (0..ray_count).map(|x| {
            let x = x as f32;
            let total_view_rad = view_rad_left - view_rad_right;
            let total_ray_divisions = (ray_count - 1) as f32;

            (view_rad_right + x * total_view_rad / total_ray_divisions).to_angle()
        }).collect::<Vec<Angle>>();
        
        println!("{:?}",ray_angles);

        let Transform(pos) = self.transform;

	
    }
}
