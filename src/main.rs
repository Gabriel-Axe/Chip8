use crate::chip8::Chip8;

mod chip8;
mod cpu;
mod memory;
mod register;
mod util;

fn main() {
    let mut chip8 = Chip8::new();
    chip8.load_rom("Blitz [David Winter].ch8");
    // chip8.draw_square();
    // chip8.draw_pixel(32, 32);
    // chip8.draw_pixel(1, 1);
    // chip8.draw_pixel(64, 1);
    // chip8.draw_pixel(64, 32);
    // chip8.draw_pixel(1, 32);
    // chip8.draw_pixel(1, 1);
    // chip8.draw_square();
    // chip8.draw_h_line();
    // chip8.draw_v_line();
    // chip8.draw_pixel_at_y(32);
    chip8.draw_line_from_u8(13, 13, 240);
    chip8.draw_line_from_u8(13, 14, 144);
    chip8.draw_line_from_u8(13, 15, 144);
    chip8.draw_line_from_u8(13, 16, 144);
    chip8.draw_line_from_u8(13, 17, 240);

    chip8.draw_line_from_u8(18, 13, 32);
    chip8.draw_line_from_u8(18, 14, 96);
    chip8.draw_line_from_u8(18, 15, 32);
    chip8.draw_line_from_u8(18, 16, 32);
    chip8.draw_line_from_u8(18, 17, 112);
    chip8.run();
}
