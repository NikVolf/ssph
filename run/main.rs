extern crate kiss3d;
extern crate nalgebra as na;
extern crate world;

use na::{Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;

fn main() {
    let mut window = Window::new("Kiss3d: cube");

    let planet = world::Planet::new(29);

    for f in 0..6 {
        for x in -14..15 {
            for y in -14..15 {
                let mut c = window.add_cube(0.05, 0.05, 0.05);
                let v = planet.warped(f, x, y);
                c.set_local_translation(na::Translation3::new(v.x as f32, v.y as f32, v.z as f32));
                c.set_color(1.0, 0.0, 0.0);
            }
        }
    }

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        //c.prepend_to_local_rotation(&rot);
    }
}