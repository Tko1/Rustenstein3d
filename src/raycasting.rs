use crate::math::Vec2f;
use crate::math::Angle;
use crate::math::ToAngle;
use rand::Rng;
use float_cmp::ApproxEq;

/*
TODO Consider representing everything with fixed point numbers instead of floats. 

I don't know if we need the inflated range of a float, but I am at a
part in the raycasting algorithm where I want to be sure that the
imprecision of floats does not cause a ray to slip through a tile. The
reasoning is when trying to jump to the next tile on a ray of light's
path, we may land very close to a corner, but not quite on it.  During
the next iteration, when trying to see what tile we're starting with,
it will have to see which of your coords (x, y, or both) falls on a
whole number -- so something like x == x.ceil(), y == y.ceil()

EXCEPT we're dealing with floats,  they aren't precise and you have to check
if they instead are very close, rather than seeing if they fall exactly on the same value.  
Its the same reason if you have two blurry photos of the same exact image,  you don't try to see 
if they blur the exact same way, you will try to approximate that their intended colors are very close.  
The blur here is the loss of precision, and we face the same thing here -- blurry boundaries, blurry corners. 

I'd rather see what happens first however before messing with it. 
*/
use self::MapEntity::{Wall,Floor};

const DEFAULT_VIEW_WIDTH : i32 = 640;

pub enum MapEntity {
    Wall,
    Floor,
    Enemy,
    Player
}

/// TODO Wrap in newtype?
/// For now,  would like to get a prototype up and running
/// and I want all the 2d vector syntax automatically for now
pub type Map = Vec<Vec<MapEntity>>;
/// 1. MapT? I am writing the trait to add functions to our Map, which
///    I didn't newtype but alised instead, but is this really right?
///    "MapT" definitely does not seem idiomatic
/// TODO just use a newtype as originally intended 
pub trait MapT {
    fn default() -> Map;
}
impl MapT for Map{
    fn default() -> Map {
	vec![vec![Wall, Wall, Wall, Wall, Wall,Wall,Wall,Wall],
	     vec![Wall, Wall,Floor, Floor, Floor,Floor,Wall,Wall],
	     vec![Wall, Floor, Wall, Floor, Wall,Floor,Floor,Wall],
	     vec![Wall, Floor, Wall, Floor, Wall,Floor,Floor,Wall],
	     vec![Wall, Floor, Wall, Floor, Wall,Floor,Floor,Wall],
	     vec![Wall, Floor, Wall, Wall, Wall,Floor,Floor,Wall],
	     vec![Wall, Floor, Floor, Floor, Floor,Floor,Floor,Wall],
	     vec![Wall, Wall, Floor, Floor, Floor,Floor,Wall,Wall],
	     vec![Wall, Wall, Wall, Wall, Wall, Wall,Wall,Wall]]
    }
}
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
            transform: Transform::new(1.1,1.1),
            rotation: Angle::new(0.0,0.0),
            horizontal_view_angle: Angle::new(1.0,1.0),
            vertical_view_angle: Angle::new(1.0,1.0),
            view_width: DEFAULT_VIEW_WIDTH
        }
    }
}
/// Just a wrapper on approx equal that bakes in the levels of precision we will use 
fn f32_equ(a: f32,b: f32) -> bool 
{
    a.approx_eq(b,(0.0,3))
}
impl Camera {
    /// For now,  everything will be thrown in here before being divvied up
    /// See documentation at docs/raycast.org 
    pub fn raycast_explicit(&self,world_map: &Map) -> Vec<(Angle,f32)>{
        let view_ang_rad = self.horizontal_view_angle.get_rad();
        let rot_rad = self.rotation.get_rad();

        // The ray angle pointing from our leftmost line of vision 
        let view_rad_right = rot_rad - (view_ang_rad / 2.0);
        // The ray angle pointing from our rightmost line of vision
        let view_rad_left = rot_rad + (view_ang_rad / 2.0);

        let ray_count = self.view_width;
        let ray_angles = (0..ray_count).map(|x| {
            let x = x as f32;
            let total_view_rad = view_rad_left - view_rad_right;
            let total_ray_divisions = (ray_count - 1) as f32;

            (view_rad_right + (ray_count as f32 - x) * total_view_rad / total_ray_divisions).to_angle()
        }).collect::<Vec<Angle>>();
        

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
        let get_ray_next_tile_cross = |pos: Vec2f,angle : Angle| -> Vec2f
        {
            // The difference is we may 'nudge' this position off a tile edge if it starts on one
            let mut starting_pos = pos;
	    
	    //let is_already_on_tile = f32_equ(px,px.ceil()) || f32_equ(py,py.ceil());
	    let already_on_tile_x = f32_equ(starting_pos.x,
					    starting_pos.x.ceil());
	    let already_on_tile_y = f32_equ(starting_pos.y,
					    starting_pos.y.ceil());
            
	    // TODO add more precise solution that won't ever go through walls
	    if already_on_tile_x || already_on_tile_y {
		
		starting_pos = starting_pos + (angle.0 * 0.00001);
	    }
	    
	    let mut px = starting_pos.x;
	    let mut py = starting_pos.y;

	    // |     |       <--- y_from_tile (and the overall rectangle this makes in the corner is the rectangle)
            // |     .----   <--- x_from_tile (and the dot is (px,py))
            // |_________|
	    //
	    // |             <--- y_from_last_tile 
            // |----.        <--- x_from_last_tile (and the dot is (px,py))
            // |____|____|
	    
            let (x_from_tile,y_from_tile) = (px.ceil() - px, py.ceil() - py);
	    let (x_from_last_tile,y_from_last_tile) = (px.floor() - px, py.floor() - py);
	    // There are 4 potential rectangle_slopes:
	    // If we are heading north to east:   
            // |     | _ /   <-- rectangle_slope  (a ray from our pos that perfectly exits through our tile corner)
            // |     /----                        If our ray is above this, it exits through the top
            // |_________|                        If our ray slopes below this, it exits through the right
	    // If we are heading north to west 
	    // |\   |    |   <-- rectangle_slope  (a ray from our pos that perfectly exits through our tile corner)
            // |___\ .   |                        If our ray is above this, it exits through the top
            // |_________|                        If our ray slopes below this, it exits through the left
	    // If we are heading west to south 
	    // |         | <-- rectangle_slope  (a ray from our pos that perfectly exits through our tile corner)
            // | ___/    |                       If our ray is above this, it exits through the left
            // |/________|                       If our ray slopes below this, it exits through the bottom
	    // If we are heading south to east
	    // |         |                       (a ray from our pos that perfectly exits through our tile corner)
            // |   \__   | <-- rectangle_slope   If our ray is above this (..well, 'below' technically since
            // |________\|                       the slopes are negative, and
	    //                                   higher rising = more negative = 'lower' slope)
	    //                                   If our ray slopes below this, it exits through the bottom
	    
            let rectangle_slope =
		//We are moving south west,  so our 'rectangle' is likewise the southwest rectangle 
		if angle.0.x < 0.0 && angle.0.y < 0.0 {     
		    y_from_last_tile / x_from_last_tile
		    //We are moving north west,  so our 'rectangle' is likewise the southwest rectangle
		} else if angle.0.x < 0.0 && angle.0.y >= 0.0 {
		     y_from_tile / x_from_last_tile
		} else if angle.0.x >= 0.0 && angle.0.y < 0.0 {
		    y_from_last_tile / x_from_tile
		} else {
		    y_from_tile / x_from_tile
		};
            let ray_slope = angle.slope();

	    // This means we will cross at the next y, or py.ceil()
	    if ray_slope > rectangle_slope && rectangle_slope >= 0.0 ||
		ray_slope < rectangle_slope && rectangle_slope < 0.0 {
		    let next_y = if angle.0.y < 0.0{ py.floor() } else { py.ceil() };
		    let next_x = y_to_x(ray_slope,starting_pos,next_y);
		    Vec2f::new(next_x,next_y)
		}
            // We cross at the next x
            else if ray_slope < rectangle_slope && rectangle_slope >= 0.0 ||
	        ray_slope > rectangle_slope && rectangle_slope < 0.0 {
		    let next_x = if angle.0.x < 0.0 { px.floor() } else { px.ceil() };
		    let next_y = x_to_y(ray_slope,starting_pos,next_x);
		    Vec2f::new(next_x,next_y)
		}
            // If ray_slope = rectangle_slope, go to px.ceil(),py.ceil()
            else {
		let mut next_x = 0.0;
		let mut next_y = 0.0;
		if angle.0.x < 0.0 {
		    next_x = px.floor();
		}
		if angle.0.x >= 0.0 {
		    next_x = px.ceil();
		}
		if angle.0.y < 0.0 {
		    next_y = py.floor();
		}
		if angle.0.y >= 0.0 {
		    next_y = py.ceil();
		}
                Vec2f::new(next_x,next_y)
		
            }
	    
        };
	ray_angles.into_iter().map(|ray_angle| {
	    let mut curr_pos = pos;
	    let mut has_collided = false;
	    let mut ray_length : f32 = 0.0;
	    // Unfortunately, some angles are getting into infinite loops, despite rendering correctly
	    // Until this bug is found, here's some safety wheels
	    let max_recur = 100;
	    let mut curr_recur = 0;
	    while !has_collided {
		curr_recur += 1;
		if curr_recur >= max_recur { break;}

		let next_cross = get_ray_next_tile_cross(curr_pos,ray_angle);
		let Vec2f{x: mut next_x, y: mut next_y} = next_cross;

		if ray_angle.0.x < 0.0 && f32_equ(next_x, next_x.floor()) {
		    next_x -= 1.0;
		}
		if ray_angle.0.y < 0.0 && f32_equ(next_y,next_y.floor()){
		    next_y -= 1.0;
		}
		
		if next_x < 0.0 { next_x = 0.0; }
		if next_y < 0.0 { next_y = 0.0; }

		
		
		let next_x_int = next_x as usize;
		let next_y_int = next_y as usize;
		
		if next_x_int >= world_map.len() ||
		    next_y_int >= world_map[0].len() ||
		    next_x_int == 0 ||
		    next_y_int == 0
		{
		    ray_length = next_cross.distance(&pos);
		    has_collided = true;
		}
		else { 
		    match world_map[next_x_int][next_y_int] {
			Wall => {
			    ray_length = next_cross.distance(&pos);
			    has_collided = true;
			},
			_ => curr_pos = next_cross,
		    };
		}
	    }
	    (ray_angle,ray_length)
	     
	}).collect::<Vec<(Angle,f32)>>()
    }
    pub fn raycast(&self,world_map: &Map) -> Vec<f32>{
	self.raycast_explicit(world_map).into_iter().map(|(_,len)| len).collect::<Vec<f32>>()
    }
    
}
