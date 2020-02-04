extern crate glium;
extern crate rand;
mod math;
use math::Vec2;
use math::Vec2f;
use math::Angle;
//use math::ToAngle;
mod raycasting;
use raycasting::*;
use raycasting::Map;
//use raycasting::MapEntity::{Wall,Floor};
use rand::Rng; 
// Graphics
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::input::*;


/*
fn returns_tuple() -> (i32, &'static str, i32){
    return (1,"Two",3);
}
*/

pub struct App {
    gl : GlGraphics,
    camera: Camera,
    key : Option<Key>,
    world_map: Map
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
	use graphics::*;
	
	const BLUE: [f32;4] = [0.0,0.0,1.0,1.0];
	const BLACK: [f32;4] = [0.0,0.0,0.0,1.0];
	
	let ray_lengths = self.camera.raycast(&self.world_map);
	let ray_ang_and_lengths = self.camera.raycast_explicit(&self.world_map);
	let mut rng = rand::thread_rng();
	////println!("rot: {:?}",self.rotation.to_degrees() % 360.0) ;
	self.gl.draw(args.viewport(), |c: Context,gl| {
	    clear(BLACK,gl);

	    let mut i = 0;
	    let transform =
		c.transform;
	    let (rotate_origin_by,_,_) = ray_ang_and_lengths[ray_ang_and_lengths.len() - 1];
	    for (ray_angle,ray_length,color) in ray_ang_and_lengths.iter() {
		let rad = ray_angle.get_rad() + ((135.0 / 2.0 as f32).to_radians() - rotate_origin_by.get_rad());
		let forward_ray_length = (rad.sin().abs() * ray_length) as f64; 
		let wall_length = 640.0 /forward_ray_length;
		let ceil_length = (640.0 - wall_length) / 2.0;
		line([0.0,0.0,0.0,1.0], 1.0,
		     [i as f64,0.0,i as f64,ceil_length],
		     transform, gl);
		line(*color, 1.0,
		     [i as f64,ceil_length,i as f64,ceil_length + wall_length],
		     transform, gl);
		line([0.0,0.0,0.0,1.0], 1.0,
		     [i as f64,wall_length + ceil_length,i as f64, 640.0],
		     transform, gl);
		i += 1;
	    }

	    for (ray_angle,ray_length,color) in ray_ang_and_lengths.iter() {
		let rad = ray_angle.get_rad() + ((45.0 / 2.0 as f32).to_radians() - rotate_origin_by.get_rad());
		let width = (rad.cos() * ray_length * 30.0) as f64;
		let height = (rad.sin() * ray_length * 30.0) as f64;
		
		line([rng.gen::<f32>(),rng.gen::<f32>(),rng.gen::<f32>(),1.0], 1.0,
		     [200.0 as f64,
		      200.0,
		      200.0 + width,
		      200.0 - height],
		     transform.trans(50.0,50.0), gl);
		i += 1;
	    }

	    
	    // transform
	    //line(GREEN, 3.0, [0.0, 0.0, 15.0, 15.0], transform, gl);
	});
	
    }

    fn update(&mut self, args: UpdateArgs) {
	let dt = args.dt as f32;
	
	if let Some(key) = self.key {
	    if key == Key::W {
		self.camera.transform.0 = self.camera.transform.0 + self.camera.rotation.0.forward() * 30.0 * dt;
	    }
	    if key == Key::S {
		self.camera.transform.0 = self.camera.transform.0 + self.camera.rotation.0.forward() * -30.0 * dt;
	    }
	    if key == Key::A {
		self.camera.transform.0 = self.camera.transform.0 + self.camera.rotation.0.forward().rotate(90.0f32.to_radians()) * 30.0 * dt;
	    }
	    if key == Key::D {
		self.camera.transform.0 = self.camera.transform.0 + self.camera.rotation.0.forward().rotate(-90.0f32.to_radians()) * 30.0 * dt;
	    }
	    if key == Key::Left {
		self.camera.rotation = self.camera.rotation.rotate(30.0 * dt);
	    }
	    if key == Key::Right {
		self.camera.rotation = self.camera.rotation.rotate(-30.0 * dt);
	    }
	    // Unfortunately, the key events come separate from the, say , update event,
	    // so for now this is my unfortunate hack of passing the key to the app when
	    // key events come in, and retrieving it / clearing it when update event comes in 
	    self.key = None;
	}
    }
    
}
fn main() {    
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("A line",[640,640])
	.graphics_api(opengl)
	.exit_on_esc(true)
	.build()
	.unwrap();
    let mut app = App {
	gl: GlGraphics::new(opengl),
	camera: Default::default(),
	world_map : <Map as MapT>::default(),
	key: None,
    };

    let mut events = Events::new(EventSettings::new());


    while let Some(e) = events.next(&mut window) {
	if let Some(Button::Keyboard(key)) = e.press_args() {
	    app.key = Some(key);
	    
	}

	if let Some(args) = e.render_args() {
	    app.render(&args);
	}

	if let Some(args) = e.update_args() {
	    app.update(args);
	}
    }    
}
