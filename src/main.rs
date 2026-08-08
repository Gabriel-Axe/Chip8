use std::{fs, sync::Arc};

use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::{LogicalSize, Pixel}, event_loop::EventLoop, window::WindowAttributes};

use crate::{chip8::Chip8, util::{convert_bytes_u8_to_u16, get_instruction_nibble, print_bytes_in_16_binary, print_bytes_in_binary}};

mod chip8;
mod cpu;
mod memory;
mod register;
mod util;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
    window: Option<Window>,
    pixels: Option<Pixel<'static>>,
}

impl Default for App {
    fn default() -> Self {
        Self { window: None, pixels: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Chip 8")
            .with_inner_size(LogicalSize::new(800, 600))
            .with_resizable(false);

        let window = Arc::new(
            event_loop
            .create_window(window_attributes)
            .expect("Failed to create window with Arc::new()"));
        println!("window created");

        let surface_texture = SurfaceTexture::new(800, 600, &window);
        let pixels = Pixels::new(800, 600, surface_texture)
            .expect("failed to create pixel buffer");
        println!("created pixel buffer");

        self.window = Some(window);
        self.pixels = Some(pixels);

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = Window
}   

fn emulator() {
    let blitz = load_rom("Blitz [David Winter].ch8");
    let mut chip8 = Chip8::new();
    chip8.interpret(&blitz);
}

fn load_rom(filename: &str) -> Vec<u16> {
    let file = fs::read(filename).expect("Could not load ROM");
    let file_as_u16 = convert_bytes_u8_to_u16(&file).expect("Could not convert ROM to u16");
    file_as_u16
}
