extern crate kiss3d;
extern crate nalgebra as na;
extern crate world;
extern crate render;

use na::Vector3;
use kiss3d::window::Window;
use kiss3d::light::Light;

fn main() {
    let mut window = Window::new("Kiss3d: cube");

    let planet = world::Planet::new(97);

    let mesh = render::planet_mesh(&planet);
    let mut node = window.add_mesh(::std::rc::Rc::new(mesh.into()), Vector3::new(1.0, 1.0, 1.0));
    node.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    while window.render() {
    }
}