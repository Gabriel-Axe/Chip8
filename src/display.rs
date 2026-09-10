use std::{fmt::format, str::FromStr};

use minifb::{Key, Window, WindowOptions};

use crate::{debug_printer::DebugPrinter, memory::{Memory, PROGRAM_START_OFFSET}, register::{Register, RegisterI}, util::{from_u8_rgb, from_u8_to_bin_color}};

const KEY_0: u8 = 0;
const KEY_1: u8 = 1;
const KEY_2: u8 = 2;
const KEY_3: u8 = 3;
const KEY_4: u8 = 4;
const KEY_5: u8 = 5;
const KEY_6: u8 = 6;
const KEY_7: u8 = 7;
const KEY_8: u8 = 8;
const KEY_9: u8 = 9;
const KEY_A: u8 = 10;
const KEY_B: u8 = 11;
const KEY_C: u8 = 12;
const KEY_D: u8 = 13;
const KEY_E: u8 = 14;
const KEY_F: u8 = 15;
const WINDOW_WIDTH: usize = 1920 / 3;
const WINDOW_HEIGHT: usize = 1080 / 3;
const BUFFER_WIDTH: usize = 64 * 3;
const BUFFER_HEIGHT: usize = 32 * 3;
// const BUFFER_WIDTH: usize = 64;
// const BUFFER_HEIGHT: usize = 32;
const BUFFER_SIZE: usize = BUFFER_WIDTH * BUFFER_HEIGHT;

pub struct Display {
    window: Window,
    buffer: Vec<u32>,
    keys: Vec<u8>,
}

fn from_u8_to_brightness(color: u32) -> u32 {
    let r = color.clone();
    let g = color.clone();
    let b = color.clone();
    from_u8_rgb(r as u8, g as u8, b as u8)
}

impl Display {

    // pub fn clear_screen(&mut self) {
    //     Display::log_display_action("clear screen".to_string());
    //     self.buffer = vec![0; BUFFER_SIZE];
    // }

    fn log_display_action(action: String) {
        DebugPrinter::log_action("display".to_string(), action);
    }

    fn convert_bool_to_color(&self, mut vec: &Vec<bool>) -> Vec<u32> {
        let mut int_vec: Vec<u32> = vec![0; vec.len()];
        let mut i = 0;
        for &val in vec {
            if val {
                int_vec[i] = from_u8_to_brightness(255);
                i += 1;
                continue;
            }
            int_vec[i] = from_u8_to_brightness(255);
            i += 1;
        };

        return int_vec
    }
    pub fn new() -> Self {
        DebugPrinter::log_info("create display".to_string());

        Display::log_display_action("create windo".to_string());
        let state = format!("window width: {} height: {}", WINDOW_WIDTH, WINDOW_HEIGHT);

        DebugPrinter::log_state(state);
        let mut window = Window::new(
            "Chip 8",
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            WindowOptions::default())
            .unwrap_or_else(|e| {
                panic!("Could not create emulator window: {}", e)
            });

        window.set_target_fps(60);

        Display::log_display_action("creating buffer".to_string());
        DebugPrinter::log_state(format!("buffer width: {} height: {}", BUFFER_WIDTH, BUFFER_HEIGHT));

        let mut buffer: Vec<u32> = vec![0; BUFFER_SIZE];

        DebugPrinter::log_state("buffer = [0; width * height]".to_string());

            Self {
                window: window,
                buffer: buffer,
            keys: vec![
                KEY_0,
                KEY_1,
                KEY_2,
                KEY_3,
                KEY_4,
                KEY_5,
                KEY_6,
                KEY_7,
                KEY_8,
                KEY_9,
                KEY_A,
                KEY_B,
                KEY_C,
                KEY_D,
                KEY_E,
                KEY_F,
            ]
            }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
    
    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, BUFFER_WIDTH, BUFFER_HEIGHT)
            .unwrap_or_else(|e| {
                panic!("Could not update display: {}", e)
            });
    }

    fn mirror_bits(&self, mut val: u8) -> u8 {
        val = ((val & 0xaa) >> 1) | ((val & 0x55) << 1);
        val = ((val & 0xcc) >> 2) | ((val & 0x33) << 2);
        val
    }

    pub fn draw_line_from_u8(&mut self, x: u8, y: u8, sprite_val: u8) -> bool {
        DebugPrinter::log_state(format!("draw line, sprite: {:04X}, x: {:}, y: {:}", sprite_val, x, y));

        let mut colided = false;
        let true_val = sprite_val;

        // NOTE:
        // When you shift a number like 44 >> 2 (or 0b101100 >> 2)
        // you get a number such as 0b1011, which after
        // AND'ng to 1 (0b1) would brigh 1 (0b101{1})
        for i in 0..8 {
            let mut pot_colide = false;
            let should_draw = (sprite_val >> i) & 1;
            // println!("should_draw {} sprite_val {} i {}", should_draw, sprite_val, i);
            if should_draw == 0 {
                // println!("not drawing");
                continue;
            }
            // println!("drawing");
            pot_colide = self.draw_pixel(x + i, y);
            // if (true_val >> i) & 1 == 1 {
            //      pot_colide = self.draw_pixel(x + i, y);
            // }
            if pot_colide == true {
                colided = pot_colide;
            }
        }

        Display::log_display_action("finish line draw".to_string());
        return colided;
    }

    pub fn draw_h_line(&mut self) {
        let start_x = 10;
        let end_x = 40;
        let y = 32;
        for x in start_x..end_x {
            self.draw_pixel(x, y);
        }
    }

    pub fn draw_v_line(&mut self) {
        let start_y = 6;
        let end_y = 28;
        let x = 54;
        for y in start_y..end_y {
            self.draw_pixel(x, y);
        }
    }

    pub fn draw_square(&mut self) {
        let start_x = 10;
        let end_x = 40;
        let start_y = 10;
        let end_y = 20;
        for x in start_x..end_x {
            for y in start_y..end_y {
                self.draw_pixel(x, y);
            }
        }
    }

    pub fn get_keys_pressed(&self) -> Vec<Key> {
        self.window.get_keys_pressed(minifb::KeyRepeat::Yes)
    }

    fn draw_pixel(&mut self, x: u8, y: u8) -> bool {
        DebugPrinter::log_state(format!("draw pixel x: {} y: {}", x, y));
        if x < 0 || x > BUFFER_WIDTH as u8 || y < 0 || y > BUFFER_HEIGHT as u8 {
            panic!("Pixel out of buffer");
        }

        let x_loc: usize = x as usize;
        let y_loc: usize = (BUFFER_WIDTH * y as usize) - BUFFER_WIDTH;
        let loc = x_loc + y_loc;
        Display::log_display_action(format!("draw pixel, x: {:}, y: {:}", x, y));
        let colided = self.invert_pixel(loc);
        self.window.update_with_buffer(&self.buffer, BUFFER_WIDTH, BUFFER_HEIGHT);
        return colided;
    }

    fn invert_pixel(&mut self, loc: usize) -> bool {
        Display::log_display_action("invert pixel".to_string());
        let old = self.buffer[loc as usize];
        let mut colided = false;
        if self.buffer[loc as usize] > 0 {
            self.buffer[loc as usize] = from_u8_to_brightness(0);
            colided = true;
        }
        else {
            self.buffer[loc as usize] = from_u8_to_brightness(255);
        }

        // let new = self.buffer[loc as usize];
        return colided;
    }

    /// Draws the pixels on the screen from the memory contents from regI data up to
    /// n_bytes, at position x and y.
    /// Returns true if any pixel colided (black to white, white to black)
    pub fn draw_sprite(&mut self, memory: &Memory, regI: &mut RegisterI, x: u8, y: u8, n_bytes: u8) {

        let n_bytes = if n_bytes == 0 {
            7
        } else {
            n_bytes
        };

        DebugPrinter::log_state(format!("regI data: 0x{:0X} x: {:}, y: {:}, n_bytes: {:}", regI.data, x, y, n_bytes));

        let y = y + 1;
        let x = x + 19;

        let starting_addr = regI.data;
        let end_addr = starting_addr + n_bytes as u16;
        let mut buffer = self.buffer.clone();
        // let mut temp = self.buffer.clone();

        // DebugPrinter::log_info(format!("buffer rn has a total of idk {} values", &val));

        DebugPrinter::log_state(format!("starting_bytes: {:04X}, end_bytes: {:04X}", starting_addr, end_addr));
        Display::log_display_action("start draw".to_string());
        for (i, addr) in (starting_addr..end_addr).enumerate() {
            // let addr = addr + PROGRAM_START_OFFSET;
            Display::log_display_action(format!("start draw line at x {} y {}", x, y));
            let mem_val = *memory.get_data_in_address(addr as usize);
            // println!("mem_val: {}", mem_val);
            let colided = self.draw_line_from_u8(x, (y + i as u8), mem_val);
            if colided {
                // WARN: Isnt it for the VF register?
                // regI.data = 1;
            }
        }

        // self.buffer = buffer;
        // self.buffer = temp;
        Display::log_display_action("finish sprite draw".to_string());
    }
}
