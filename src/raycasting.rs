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

	
	// We are flat out using the ol mx + y0 = y to represent our ray on the map
        // This is just the inverse function, to get x given a y 
        let y_to_x = |ray_slope,pos,y| {
	    let Vec2f {x: px, y: py} = pos;
	    // y intercept; See notes, this can be derived with your old mx + y0 = y
            // and will be used to create a f(x) = y,  and a f(y) = x  
	    let y0 = py - ray_slope * px;
	    (y - y0)/ray_slope
        };
	
	// We are flat out using the ol mx + y0 = y to represent our ray on the map
        // So we can get a y given an x
        let x_to_y = |ray_slope,pos,x| {
	    let Vec2f {x: px, y: py} = pos;
	    // y intercept; See notes, this can be derived with your old mx + y0 = y
            // and will be used to create a f(x) = y,  and a f(y) = x  
	    let y0 = py - ray_slope * px;
	    ray_slope * x + y0
        };
        let get_ray_next_tile_cross = |map: Map,pos: Vec2f, angle : Angle| -> Vec2f
        {
            
            let Vec2f {x: px, y: py} = pos;
            // |     |       <--- y_from_tile (and the overall rectangle this makes in the corner is the rectangle)
            // |     .----   <--- x_from_tile (and the dot is (px,py))
            // |_________|   
            let (x_from_tile,y_from_tile) = (px.ceil() - px, py.ceil() - py);
            // |     | _ /   <-- rectangle_slope  (a ray from our pos that perfectly exits through our tile corner)
            // |     /----                        If our ray is above this, it exits through the top
            // |_________|                        If our ray slopes below this, it exits through the right
            let rectangle_slope = y_from_tile / x_from_tile;
            let ray_slope = angle.slope();

	    // This means we will cross at the next y, or py.ceil()
	    if ray_slope > rectangle_slope {
		let next_x = y_to_x(ray_slope,pos,py.ceil());
		return Vec2f::new(next_x,py.ceil());
            }
            // We cross at the next x
            else if ray_slope < rectangle_slope {
		let next_y = x_to_y(ray_slope,pos,px.ceil());
		return Vec2f::new(px.ceil(),next_y);
            }
            // If ray_slope = rectangle_slope, go to px.ceil(),py.ceil()
            else { 
                return Vec2f::new(px.ceil(),py.ceil());
            }

        };
	let ray_lengths = ray_angles.into_iter().map(|x| {
	    
	}).collect::<Vec<()>>();
        let mut ray_length = 0;
	/*
	for ray_angle in ray_angles {
	    let ray_slope = ray_angle.slope();	    
	    
	}
	 */
    }
}
