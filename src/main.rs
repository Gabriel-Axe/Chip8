use minifb::{Key, Window, WindowOptions};

mod chip8;
mod cpu;
mod memory;
mod register;
mod util;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn main() {

    let window_width = 800;
    let window_height = 600;
    let buffer_width = 40;
    let buffer_height = 30;

    let mut window = Window::new(
        "Test - ESC to exit",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Could not create window: {}", e);
    });

    window.set_target_fps(60);

    let azure_blue = from_u8_rgb(0, 127, 255);
    let mut buffer: Vec<u32> = vec![azure_blue; buffer_width * buffer_height];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // buffer.fill(from_u8_rgb(0, 0, 0));
        // fill_window_buffer(&mut buffer, 255);
        fill_buffer_with_color(&mut buffer, 44);

        window
            .update_with_buffer(&buffer, buffer_width, buffer_height)
            .unwrap_or_else(|e| { 
                    panic!("Failed to update window: {}", e)
            });
        }
}

fn fill_buffer_with_color(buffer: &mut Vec<u32>, color: u8) {
    buffer.fill(from_u8_rgb(color, color, color));
}
