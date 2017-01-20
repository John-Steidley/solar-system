extern crate kiss3d;
extern crate nalgebra as na;

use na::Vector3;
fn main() {
    let mut window = kiss3d::window::Window::new("Solar system");
    window.set_light(kiss3d::light::Light::StickToCamera);

    let sun_radius = 0.1f32; // arbitrary unit to fit in initial camera view
    let mut sun = window.add_sphere(sun_radius);
    sun.set_color(0.95, 0.7, 0.0);

    let venus_radius = sun_radius * 0.25;
    let mut venus = window.add_sphere(venus_radius);
    venus.set_color(1.0, 0.3, 0.0);

    let earth_radius = sun_radius * 0.25;
    let mut earth = window.add_sphere(earth_radius);
    earth.set_color(0.0, 0.8, 1.0);

    let moon_radius = earth_radius * 0.25;
    let mut moon = window.add_sphere(moon_radius);
    moon.set_color(0.2, 0.2, 0.2);

    let distance_sun_to_venus = sun_radius * 3.;
    let distance_sun_to_earth = sun_radius * 5.0;
    let distance_earth_to_moon = earth_radius * 2.0;

    let mut day = 1;
    loop {
        day += 1;
        let sun_coords = Vector3::new(0.0, 0.0, 0.0);

        let venus_coords = get_coordinates(day, 3000, distance_sun_to_venus, &sun_coords);
        let earth_coords = get_coordinates(day, 5000, distance_sun_to_earth, &sun_coords);
        let moon_coords = get_coordinates(day, 500, distance_earth_to_moon, &earth_coords);

        venus.set_local_translation(venus_coords);
        earth.set_local_translation(earth_coords);
        moon.set_local_translation(moon_coords);

        if !window.render() {
            break;
        }
    }
}

fn get_coordinates(day: u64, orbit_period: u64, scale: f32, origin: &Vector3<f32>) -> Vector3<f32> {
    let fraction = (day % orbit_period) as f32 / orbit_period as f32;
    let in_rads = fraction * std::f32::consts::PI * 2.0f32;
    let x = in_rads.sin() * scale;
    let y = 0.0;
    let z = in_rads.cos() * scale;
    return Vector3::new(x, y, z) + *origin;
}