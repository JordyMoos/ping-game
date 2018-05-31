extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};
use std::time::{Duration, SystemTime};
use std::thread::sleep;
//use sdl2::image::{LoadTexture, INIT_PNG};
use sdl2::gfx::primitives::DrawRenderer;


const TEXTURE_SIZE: u32 = 32;


#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
}


pub fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Failed to convert window into canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let green_square = create_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::Green,
        TEXTURE_SIZE)
        .expect("Failed to create a texture");
    let blue_square = create_texture_rect(
            &mut canvas,
            &texture_creator,
            TextureColor::Blue,
            TEXTURE_SIZE)
        .expect("Failed to create a texture");

    //let _image_context = sdl2::image::init(INIT_PNG)
    //    .expect("Couldn't initialize image creator");
    // let image_texture = texture_creator.load_texture("assets/cat.png")
    //    .expect("Couldn't load image");

    let timer = SystemTime::now();

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // canvas.copy(&image_texture, None, None).expect("Render image failed");

        let square_texture = match timer.elapsed() {
            Ok(elapsed) =>
                if elapsed.as_secs() % 2 == 0  {
                    &green_square
                } else {
                    &blue_square
                },
            Err(_) => &blue_square,
        };

        // Arena
        canvas.filled_circle(400, 300, 200, Color::RGB(0, 0, 255)).unwrap();
        canvas.filled_circle(400, 300, 180, Color::RGB(0, 0, 0)).unwrap();
        
        // Goal
        canvas.filled_pie(400, 300, 202, 0, 45, Color::RGB(0, 0, 0)).unwrap();

        // Player
        canvas.filled_pie(400, 300, 160, 10, 30, Color::RGB(0, 255, 0)).unwrap();
        canvas.filled_pie(400, 300, 155, 9, 31, Color::RGB(0, 0, 0)).unwrap();
        
        // Ball
        canvas.filled_circle(500, 350, 5, Color::RGB(255, 255, 0)).unwrap();

        canvas.present();

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    color: TextureColor,
    size: u32
) -> Option<Texture<'a>> {
    match texture_creator.create_texture_target(None, size, size) {
        Ok(mut square_texture) => {
            canvas.with_texture_canvas(&mut square_texture, |texture| {
                match color {
                    TextureColor::Green =>
                        texture.set_draw_color(Color::RGB(0, 255, 0)),
                    TextureColor::Blue =>
                        texture.set_draw_color(Color::RGB(0, 0, 255)),
                }

                texture.clear();
            }).expect("Failed to color a texture");

            Some(square_texture)
        },
        _ => None,
    }
}
