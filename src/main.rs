use std::{fs, sync::Arc};

use minifb::{Key, Window, WindowOptions};
use pixels::{Pixels, SurfaceTexture};
// use winit::{dpi::{LogicalSize, Pixel}, event_loop::EventLoop, window::WindowAttributes};

// use crate::{chip8::Chip8, util::{convert_bytes_u8_to_u16, get_instruction_nibble, print_bytes_in_16_binary, print_bytes_in_binary}};

mod chip8;
mod cpu;
mod memory;
mod register;
mod util;

// struct App {
//     window: Option<Arc<Window>>,
//     pixels: Option<Pixels<'static>>,
// }

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;
const BUFFER_WIDTH: usize = 400;
const BUFFER_HEIGHT: usize = 600;
// const SCALE: usize = 10;
// const WIDTH: usize = 64;
// const HEIGHT: usize = 32;
// const SCALE: usize = 10;

// impl Default for App {
//     fn default() -> Self {
//         Self { window: None, pixels: None }
//     }
// }
//
// impl ApplicationHandler for App {
//     fn resumed(&mut self, event_loop: &ActiveEventLoop) {
//         let window_attributes = Window::default_attributes()
//             .with_title("Chip 8")
//             .with_inner_size(LogicalSize::new(800, 600))
//             .with_resizable(false);
//             let window = Arc::new(
//
//             event_loop
//             .create_window(window_attributes)
//             .expect("Failed to create window with Arc::new()"));
//         println!("window created");
//         self.window = Some(window);
//
//         let surface_texture = SurfaceTexture::new(800, 600, window);
//         // let pixels = Pixels::new(800, 600, surface_texture)
//         //     .expect("failed to create pixel buffer");
//         // println!("created pixel buffer");
//         //
//         // self.pixels = Some(pixels);
//         //
//         // if let Some(window) = &self.window {
//         //     window.request_redraw();
//         // }
//     }
//
//     fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
//         if let Some(window) = &self.window {
//             window.request_redraw();
//         }
//     }
//
// fn window_event(
//         &mut self,
//         event_loop: &ActiveEventLoop,
//         _window_id: WindowId,
//         event: WindowEvent,
//     ) {
//         match event {
//             WindowEvent::CloseRequested => {
//                 println!("👋 Closing CHIP-8");
//                 event_loop.exit();
//             }
//             WindowEvent::RedrawRequested => {
//                 if let (Some(window), Some(pixels)) = (&self.window, &mut self.pixels) {
//                     let frame = pixels.frame_mut();
//
//                     // Clear to black
//                     for pixel in frame.chunks_exact_mut(4) {
//                         pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xFF]);
//                     }
//
//                     // Draw test pattern - white border
//                     for x in 0..WIDTH {
//                         for y in 0..HEIGHT {
//                             if x == 0 || x == WIDTH - 1 || y == 0 || y == HEIGHT - 1 {
//                                 let idx = (y * WIDTH + x) as usize;
//                                 let pixel = &mut frame[idx * 4..(idx + 1) * 4];
//                                 pixel.copy_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]);
//                             }
//                         }
//                     }
//
//                     // Draw red pixel in center
//                     let idx = ((HEIGHT/2) * WIDTH + (WIDTH/2)) as usize;
//                     let pixel = &mut frame[idx * 4..(idx + 1) * 4];
//                     pixel.copy_from_slice(&[0xFF, 0x00, 0x00, 0xFF]);
//
//                     // Render to screen
//                     if let Err(e) = pixels.render() {
//                         eprintln!("Failed to render: {}", e);
//                     }
//
//                     // Request next frame
//                     window.request_redraw();
//                 }
//             }
//             _ => {}
//         }
//     }
// }

fn rgb(r: u8, g:u8, b:u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn new_window() {
    
}

fn main() {

    let mut window = Window::new(
        "Test - ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("AAAAAAAAAAAAAAAAAAAAAAAAAA {}", e);
    });

    let mut buffer: Vec<u32> = vec![0; BUFFER_WIDTH * BUFFER_HEIGHT];
    window.set_target_fps(60);

    let mut cur_red = 0;
    let mut cur_blue = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(rgb(0, 0, 0));
        fill_window_buffer(&mut buffer, 255);
        let center_x = WINDOW_WIDTH / 2;
        let center_y = WINDOW_HEIGHT / 2;
        // buffer[center_y * WIDTH + center_x ] = rgb(255, 0, 0);
        // buffer[center_y + center_x ] = rgb(255, 0, 0);
        // buffer[center_y * WIDTH + center_x ] = rgb(255, 0, 0);

        window
            .update_with_buffer(&buffer, BUFFER_WIDTH , WINDOW_HEIGHT )
            .expect("Failed to update winodw");
        }
}

// fn fill_window_buffer(buffer: &mut Vec<u32>, width: usize, height: usize, color: u8) {
fn fill_window_buffer(buffer: &mut Vec<u32>, color: u8) {
    let x_loop = BUFFER_WIDTH - 1;
    let y_loop = BUFFER_HEIGHT - 1;
    for x in 0..x_loop {
        for y in 0..y_loop {
            if x == 0 || x == BUFFER_WIDTH - 1 || y == 0 || y == BUFFER_HEIGHT - 1 {
                // buffer[y * WIDTH + x] = rgb(color, color, color);
                buffer[y] = rgb(255, 0, 0);
            }
            // cur_blue += 1;
        }
        // cur_red += 1;
    }
}

fn emulator() {
    // let blitz = load_rom("Blitz [David Winter].ch8");
    // let mut chip8 = Chip8::new();
    // chip8.interpret(&blitz);
}

// fn load_rom(filename: &str) -> Vec<u16> {
//     // let file = fs::read(filename).expect("Could not load ROM");
//     // let file_as_u16 = convert_bytes_u8_to_u16(&file).expect("Could not convert ROM to u16");
//     // file_as_u16
// }
