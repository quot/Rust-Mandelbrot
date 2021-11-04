extern crate sdl2;

use num::ToPrimitive;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::cmp::min;
use sdl2::rect::Point;
use num::complex::Complex;

fn mandelbrot_value_at_point(
    cx: f64,
    cy: f64,
    max_iters: i32
) -> i32 {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 20.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters
}

pub fn main() {
    let mandl_x_min: f64 = -2.0;
    let mandl_x_max: f64 = 1.0;
    let mandl_y_min: f64 = -1.0;
    let mandl_y_max: f64 = 1.0;
    let win_width: u32 = 800;
    let win_height: u32 = 600;
    let max_iters: i32 = 1000;

    //===========
    // SDL Setup
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Mandelbrot", win_width, win_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    //==============
    // Drawing Loop
    let mut cur_y: u32 = 0;
    'running: loop {
        // Check for quit events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        //==================
        // START MANDELBROT
        if cur_y < win_height {
            for cur_x in 0..win_width {
                // Setup for call to calculate
                let x_percent = cur_x as f64 / win_width as f64;
                let y_percent = cur_y as f64 / win_height as f64;
                let cx = mandl_x_min + (mandl_x_max - mandl_x_min) * x_percent;
                let cy = mandl_y_min + (mandl_y_max - mandl_y_min) * y_percent;
                let mandelbrot_val = mandelbrot_value_at_point(cx, cy, max_iters);
                //if (mandelbrot_val < 1000 && mandelbrot_val > 0) { println!("MandVal: {}", mandelbrot_val) }

                let rgb_vals: Color = match mandelbrot_val {
                    0 => Color::RGB(0, 0, 0),
                    1 => Color::RGB(0, 0, 50),
                    2 => Color::RGB(0, 0, 100),
                    3 => Color::RGB(0, 0, 150),
                    4 => Color::RGB(0, 0, 200),
                    5 => Color::RGB(0, 0, 220),
                    6..=7 => Color::RGB(0, 0, 255),
                    8..=9 => Color::RGB(20, 20, 255),
                    10..=20 => Color::RGB(40, 40, 255),
                    21..=30 => Color::RGB(70, 70, 255),
                    31..=50 => Color::RGB(100, 100, 255),
                    51..=100 => Color::RGB(150, 150, 255),
                    101..=999 => Color::RGB(200, 200, 255),
                    _ => Color::RGB(255,255,255)
                };
                // Draw calculated value
                canvas.set_draw_color(rgb_vals);
                canvas.draw_point(Point::new(cur_x.to_i32().unwrap(), cur_y.to_i32().unwrap()));
            }
            cur_y += 1;
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
