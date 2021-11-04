extern crate sdl2;

use num::complex::Complex;
use num::{Float, ToPrimitive};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::cmp::min;
use std::time::Duration;

fn mandelbrot_color_at_point(cx: f64, cy: f64, max_iters: usize) -> Color {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 50.0 {
            let mut v: f64 = (i as f64 + 1.5 - z.norm().abs().log10().log2()).log10() / 3.4;
            if v < 1.0 {
                return Color::RGB(
                    min(255, (255.0 * v.powf(4.0)).round() as u8),
                    min(255, (255.0 * v.powf(2.5)).round() as u8),
                    min(255, (255.0 * v).round() as u8),
                );
            } else {
                v = (2.0 - v).max(0.0);
                return Color::RGB(
                    min(255, (255.0 * v).round() as u8),
                    min(255, (255.0 * v.powf(1.5)).round() as u8),
                    min(255, (255.0 * v.powf(3.0)).round() as u8),
                );
            }
        }
        z = z * z + c;
    }
    Color::RGB(0, 0, 0)
}

pub fn main() {
    let win_width: u32 = 1920;
    let win_height: u32 = 1300;
    let mandl_x_min: f64 = -2.1;
    let mandl_x_max: f64 = 0.8;
    let mandl_y_min: f64 =
        ((mandl_x_min - mandl_x_max) * 0.5 * win_height as f64) / win_width as f64;
    let mandl_y_max: f64 = (0.0 - mandl_y_min as f64) + 0.01;
    let max_iters: usize = 500;

    //===========
    // SDL Setup
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Mandelbrot", win_width, win_height)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    //==============
    // Drawing Loop
    let mut cur_y: u32 = 0;
    'running: loop {
        // Check for quit events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
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

                // Draw calculated value
                canvas.set_draw_color(mandelbrot_color_at_point(cx, cy, max_iters));
                canvas
                    .draw_point(Point::new(cur_x.to_i32().unwrap(), cur_y.to_i32().unwrap()))
                    .unwrap();
            }
            cur_y += 1;
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
