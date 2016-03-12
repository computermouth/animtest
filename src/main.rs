extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("SLAPFIGHT", 640, 440)
        .position_centered()
        .build()
        .unwrap();

    let mut renderer = window.renderer()
		.present_vsync()
		.build()
		.unwrap();
		
		
	'running: loop {
		
		renderer.set_draw_color(Color::RGBA(30, 30, 39, 255));
		renderer.clear();
		let mut event_pump = sdl_context.event_pump().unwrap();
		
		renderer.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}  => {
                    break 'running
                },
                _ => {}
            }
        }
    }
    
    
}
