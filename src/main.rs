extern crate kiss3d;
extern crate nalgebra as na;

use na::Vector3;
use kiss3d::window::Window;
use kiss3d::light::Light;

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    window.set_light(Light::StickToCamera);

    let sun_size = 1.0f32;
    let mut sun = window.add_sphere(sun_size);
    sun.set_color(1.0, 1.0, 0.0);

    let planet_size = sun_size / 30.0;
    let mut planet = sun.add_sphere(planet_size);
    let distance_sun_to_planet = sun_size * 10.0;
    planet.prepend_to_local_translation(&Vector3::new(distance_sun_to_planet, 0.0, 0.0));

    let moon_size = planet_size;
    let mut moon = planet.add_sphere(moon_size);
    let distance_planet_to_moon = planet_size * 3.0;
    moon.prepend_to_local_translation(&Vector3::new(distance_planet_to_moon, 0.0, 0.0));

    let mut counter = 0;
    while window.render() {
        sun.prepend_to_local_rotation(&Vector3::new(0.0f32, 0.0005, 0.0));

        if counter == 200 {
            counter = 0;
        } else {
            counter += 1;
        }
        let factor = counter as f32 / 200.0;

        let red = 0.0;
        let green = factor;
        let blue = 1.0 - factor;
        planet.prepend_to_local_rotation(&Vector3::new(0.0f32, 0.002, 0.0));
        planet.set_color(red, green, blue);
        moon.set_color(0.2, 0.2, 0.2);
    }
}