#[macro_use]
extern crate glium;
extern crate cgmath;

mod vertex;
mod camera;
mod planet;
mod triangle;
mod renderable;

fn main() {
    use glium::{DisplayBuild, Surface};
    use vertex::{VertexPosition, Normal};
    use camera::Camera;
    use cgmath::Vector3;
    use planet::Planet;

    let display = glium::glutin::WindowBuilder::new().with_depth_buffer(24).build_glium().unwrap();
    let target = display.draw();
    let (width, height) = target.get_dimensions();
    target.finish().unwrap();

    let mut cam : Camera = Camera::new(Vector3::new(0.0, -5.0, 0.0), width as f32 / height as f32, 5.);
    let mut planet : Planet<VertexPosition, Normal> = Planet::new(&display, 3);

    let mut vertical_position : f32 = 0.;
    let mut horisontal_angle : f32 = 0.;

    let light = [-1.0, 0.4, 0.9f32];

    loop {
        let mut target = display.draw();
        
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
        .. Default::default()
        };

        planet.draw(&mut target, params, cam.view, cam.projection, light);
        target.finish().unwrap();


        let mut cam_pos = cam.get_position();
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                glium::glutin::Event::Closed => return,

                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Up)) => vertical_position += 0.1,
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Down)) => vertical_position -= 0.1,

                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Right)) => horisontal_angle += 0.1,
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Left)) => horisontal_angle -= 0.1,

                _ => ()
            }
        }

        if vertical_position < -1.{
            vertical_position = -1.;
        }else if vertical_position > 1.{
            vertical_position = 1.;}

        cam_pos += Vector3::new(f32::cos(horisontal_angle), f32::sin(horisontal_angle), vertical_position);
        cam.set_position(cam_pos);
    }
}