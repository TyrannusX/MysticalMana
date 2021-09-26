//define external crates (pre-built binaries)
extern crate sdl2;

//module imports
use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::image::*;
use sdl2::rect::{Rect, Point};
use sdl2::video::{Window, WindowContext};
use sdl2::render::{WindowCanvas, TextureCreator, Texture};

fn main() {
    /*
    Define contexts that allow you to interect with OS and graphics
    */
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    //Define window handle that abstracts OS level windowing shit
    let window: Window = video_subsystem.window("Rust SDL2", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    //Define a canvase inside the window (basically where you draw stuff to)
    let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    //Define texture creator to render images
    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

    //Define character
    let main_character: Texture = texture_creator.load_texture("C:\\GameAssets\\DungeonTileset\\frames\\wizzard_m_idle_anim_f0.png").unwrap();

    /*
        Madness explained:
        1. Get the total width and height of the window
        2. Define the area of the window to start drawing texture (main_character)
            *in this case we want the starting point to be center of screen
            *to get center of screen, do rectangle of 0,0 and a rectangle of (width/2, height/2)
        3. Define source area (part of texture) to take.
            *Helpful with sprite sheets to grab 1 image at a time
            *For a normal PNG, lets you control scale
        4. Define destination area (place on window/canvas) to draw
            *In this case we start drawing from center of screen and draw selected area of PNG
    */

    let (width, height) = canvas.output_size().unwrap(); 
    let mut starting_point: Point = Point::new(0, 0) + Point::new(width as i32 / 2, height as i32 / 2);
    let main_character_cut_off_image_rect: Rect = Rect::new(0, 0, 64, 128);
    let mut screen_rect: Rect = Rect::from_center(starting_point, main_character_cut_off_image_rect.width(), main_character_cut_off_image_rect.height());

    //Setup SDL events (control workflow of game)
    let mut event_pump: EventPump = sdl_context.event_pump().unwrap();
    let mut i: u8 = 0;
    'gameloop: loop {
        //change the color over time
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - 1));
        canvas.clear();

        //process events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => { break 'gameloop; },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'gameloop; },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => { starting_point.x += 10 }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => { starting_point.x -= 10 }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => { starting_point.y -= 10 }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => { starting_point.y += 10 }
                _ => { }
            }
        }

        /*
            Using Rectangles defined above, copy sprite 
            to canvas/screen/window (whatever the hell you wanna call it)
        */
        screen_rect = Rect::from_center(starting_point, main_character_cut_off_image_rect.width(), main_character_cut_off_image_rect.height());
        canvas.copy(&main_character, main_character_cut_off_image_rect, screen_rect).unwrap();
        canvas.present();

        //Put main thread to sleep for half a second?
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
