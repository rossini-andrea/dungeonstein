extern crate nalgebra as na;
extern crate nalgebra_glm as glm;

mod rendering;
mod game;
mod uglythings;

use glm::{Vec3};
use sdl2::{
    image::InitFlag,
    event::Event,
    keyboard::Keycode
};
use rendering::graphics::{ GlEngine, ViewSettings };

use std::time::{ SystemTime, Duration };

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    sdl2::image::init(InitFlag::PNG).unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Window", 800, 600)
        .opengl()
        .build()
        .unwrap();

    let mut canvas = match window.into_canvas().index(find_sdl_gl_driver().unwrap())
        .build() {
        Ok(x) => x,
        Err(e) => panic!("{}", e)
    };
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.window().gl_set_context_to_current().unwrap();
    let texture_creator = canvas.texture_creator();
    // Push the previous code into the Engine??
    let mut renderer = GlEngine::new(&texture_creator);
    let mut game = uglythings::build_experimental_game();

    unsafe {
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthMask(gl::TRUE);
        gl::DepthFunc(gl::LESS);
    }

    let mut frames = 0i32;
    let mut start = SystemTime::now();

    'running: loop {
        game.player.speed = Vec3::new(0.0, 0.0, 0.0);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => {
                    break 'running
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Up), ..
                } => {
                    game.player.speed = game.player.direction * 0.75;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Down), ..
                } => {
                    game.player.speed = -game.player.direction * 0.75;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Left), ..
                } => {
                    game.player.angle += 3.14 / 16.0;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Right), ..
                } => {
                    game.player.angle -= 3.14 / 16.0;
                }
                _ => {}
            }

            game.player.direction = glm::rotate_z_vec3(
                &Vec3::new(1.0, 0.0, 0.0), game.player.angle
            );
        }

        game.player.pos += game.player.speed;

        canvas.window().gl_set_context_to_current().unwrap();

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        renderer.render(
            &game.current_map, ViewSettings {
                pos: game.player.pos,
                facing: game.player.direction,
                height: 1.77
            }
        );
        canvas.present();

        frames += 1;

        if start.elapsed().unwrap_or(Duration::from_secs(0)).as_secs() >= 10 {
            println!("{} FPS", frames / 10);
            start = SystemTime::now();
            frames = 0;
        }
    }
}
