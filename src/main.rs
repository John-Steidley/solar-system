extern crate kiss3d;

use std::collections::HashMap;
use kiss3d::window::Window;
use kiss3d::scene::SceneNode;
use kiss3d::nalgebra::Translation3;


struct Body<'a> {
    name: &'a str,
    radius: f32,
    orbit_period: u64,
    orbit_radius: f32,
    red: f32,
    green: f32,
    blue: f32,
    satelites: Option<Vec<Body<'a>>>,
}

fn main() {
    let mut window = Window::new("Solar system");
    window.set_light(kiss3d::light::Light::StickToCamera);

    let solar_system: Body = Body{
        name: "Sun",
        radius: 0.1,
        orbit_period: 1,
        orbit_radius: 0.0,
        red: 0.95,
        green: 0.7,
        blue: 0.0,
        satelites: Some(vec![
            Body{
                name: "Mercury",
                radius:0.010,
                orbit_period: 500,
                orbit_radius: 0.2,
                red: 0.2,
                green: 0.2,
                blue: 0.2,
                satelites: None,
            },
            Body{
                name: "Venus",
                radius: 0.025,
                orbit_period: 3000,
                orbit_radius: 0.3,
                red: 1.0,
                green: 0.3,
                blue: 0.0,
                satelites: None,
            },
            Body{
                name: "Earth",
                radius: 0.025,
                orbit_period: 5000,
                orbit_radius: 0.5,
                red: 0.0,
                green: 0.8,
                blue: 1.0,
                satelites: Some(vec![
                    Body{
                        name: "The Moon",
                        radius: 0.005,
                        orbit_period: 500,
                        orbit_radius: 0.075,
                        red: 0.2,
                        green: 0.2,
                        blue: 0.2,
                        satelites: None,
                    }
                ]),
            },
            Body{
                name: "Mars",
                radius: 0.013,
                orbit_period: 4000,
                orbit_radius: 0.7,
                red: 1.0,
                green: 0.3,
                blue: 0.0,
                satelites: None,
            },
            Body{
                name: "Jupiter",
                radius:0.080,
                orbit_period: 10000,
                orbit_radius: 1.1,
                red: 0.5,
                green: 0.3,
                blue: 0.0,
                satelites:None,
            },
            Body{
                name: "Saturn",
                radius: 0.060,
                orbit_period: 12000,
                orbit_radius:1.3,
                red: 0.5,
                green: 0.3,
                blue:0.0,
                satelites:None,
            },
            Body{
                name: "Uranus",
                radius: 0.045,
                orbit_period: 30000,
                orbit_radius: 2.6,
                red: 0.3,
                green: 0.8,
                blue: 1.0,
                satelites:None,
            },
            Body{
                name:"Neptune",
                radius: 0.045,
                orbit_period: 35000,
                orbit_radius: 2.9,
                red: 0.0,
                green: 0.0,
                blue: 0.5,
                satelites:None,
            },
        ]),
    };

    let mut nodes = HashMap::new();
    add_bodies(&mut window, &solar_system, &mut nodes);
    let origin = Translation3::new(0.0, 0.0, 0.0);
    let mut day = 78920;
    while window.render() {
        day += 1;
        move_bodies(&mut nodes, &solar_system, &origin, day);
    }
}

fn add_bodies(window: &mut Window, body: &Body, nodes: &mut HashMap<String, SceneNode>) {
    let mut handle = window.add_sphere(body.radius);
    handle.set_color(body.red, body.green, body.blue);
    nodes.insert(body.name.to_string(), handle);
    if let Some(ref satelites) = body.satelites {
        for b in satelites {
            add_bodies(window, &b, nodes);
        }
    }
}

fn move_bodies(nodes: &mut HashMap<String, SceneNode>, body: &Body, origin: &Translation3<f32>, day: u64) {
    let coords = get_coordinates(day, body.orbit_period, body.orbit_radius, origin);
    let mut handle = nodes.remove(&body.name.to_string()).unwrap();
    if let Some(ref satelites) = body.satelites {
        for b in satelites {
            move_bodies(nodes, &b, &coords, day);
        }
    }
    handle.set_local_translation(coords);
    nodes.insert(body.name.to_string(), handle);
}

fn get_coordinates(day: u64, orbit_period: u64, scale: f32, origin: &Translation3<f32>) -> Translation3<f32> {
    let fraction = (day % orbit_period) as f32 / orbit_period as f32;
    let in_rads = fraction * std::f32::consts::PI * 2.0f32;
    let x = in_rads.sin() * scale + origin.x;
    let y = 0.0 + origin.y;
    let z = in_rads.cos() * scale + origin.z;
    return Translation3::new(x, y, z);
}